

use std::io::Write;
use std::str::FromStr;

use chrono::NaiveDateTime;

use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::serialize::ToSql;
use diesel::pg::{Pg, PgValue};
use diesel::{expression::AsExpression, prelude::*};
use diesel::sql_types::Text;
use crate::schema::*;
use serde::{Serialize, Deserialize};



#[derive(Queryable, Serialize, Deserialize)]
pub struct DataSiswa  {
    #[serde(skip_deserializing)]
    pub id: i32, 
    pub name: String,
    pub kelas: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=datasiswa)]
pub struct NewDataSiswa  { 
    pub name: String,
    pub kelas: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct NilaiSiswa  {
    #[serde(skip_deserializing)]
    pub id: i32, 
    pub datasiswa_id: i32,
    pub nilai: String,
    pub name: String,
    pub matapelajaran: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=nilaisiswa)]
pub struct NewNilaiSiswa  {  
    pub datasiswa_id: i32,
    pub nilai: String,
    pub name: String,
    pub matapelajaran: String,
    pub description: Option<String>
}

#[derive(Queryable, Debug, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime
}
#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime
}


#[derive(Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct  UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct  NewUserRole {
    pub role_id: i32,
    pub user_id: i32,
}

#[derive(AsExpression, Debug, FromSqlRow)]
#[diesel(sql_type =Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => String::from("admin"),
            RoleCode::Editor => String::from("editor"),
            RoleCode::Viewer => String::from("viewer"),
        }
    }
}

impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err({}),
        }
    }
}

impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"editor" => Ok(RoleCode::Editor),
            b"viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer),
        }
    }
}

impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
        };
        Ok(diesel::serialize::IsNull::No)
    }
}




