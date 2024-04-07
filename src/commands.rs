
use std::str::FromStr;

use chrono::{Utc, Datelike};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use tera::{Tera,Context};

use crate::auth;
use crate::mail::HtmlMailer;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{NilaiSiswaRepository, RoleRepository, UserRepository};



fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html")
        .expect("Cannot load template engine")
}


async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut db_conn = load_db_connection().await;
    
    
    let password_hash = auth::hash_password(password).unwrap();   
    let new_user = NewUser{ username, password: password_hash.to_string()};
    let role_enums = role_codes.iter().map(|v| RoleCode::from_str(v.as_str()).unwrap()).collect();
    let user = UserRepository::create(&mut db_conn, new_user, role_enums).await.unwrap();
    println!("User Created{:?}", user);
    let roles = RoleRepository::find_by_user(&mut db_conn, &user).await.unwrap();
    println!("Roles assigned{:?}", roles);
}

pub async fn list_users() {
    let mut db_conn = load_db_connection().await;
    
    let users = UserRepository::find_with_roles(&mut db_conn).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_user(id:i32) {
    let mut db_conn = load_db_connection().await;

    UserRepository::delete(&mut db_conn, id).await.unwrap();
}

pub async fn digest_send(email: String, hours_since: i32) {
    let mut c = load_db_connection().await;
    let tera = load_template_engine();

    let nilaisiswa = NilaiSiswaRepository::find_since(&mut c, hours_since).await.unwrap();
    if nilaisiswa.len() > 0 {
        println!("Sending digest for {} Nilai Siswa", nilaisiswa.len());
        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &nilaisiswa);
        context.insert("year", &year);

        let smtp_host = std::env::var("SMTP_HOST")
            .expect("Cannot load SMTP host from environment");
        let smtp_username = std::env::var("SMTP_USERNAME")
            .expect("Cannot load SMTP username from environment");
        let smtp_password = std::env::var("SMTP_PASSWORD")
            .expect("Cannot load SMTP password from environment");

        let mailer = HtmlMailer { template_engine: tera, smtp_host, smtp_username, smtp_password };
        mailer.send(email, "email/digest.html", context).unwrap();
    }
}

