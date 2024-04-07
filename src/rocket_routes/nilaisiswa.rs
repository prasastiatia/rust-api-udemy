use crate::models::NilaiSiswa;
use crate::models::NewNilaiSiswa;
use crate::models::User;
use crate::repositories::NilaiSiswaRepository;


use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::rocket_routes::{DbConn, server_error};

#[rocket::get("/nilai-siswa")]
pub async fn get_nilai(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    NilaiSiswaRepository::find_multiple(&mut db, 100).await
        .map(|nilaisiswa| json!(nilaisiswa))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/nilai-siswa/<id>")]
pub async fn view_nilai( mut db: Connection<DbConn>, id:i32, _user: User) -> Result<Value,Custom<Value>> {
    NilaiSiswaRepository::find(&mut db, id).await
        .map(|nilaisiswa| json!(nilaisiswa))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/nilai-siswa", format="json", data="<new_nilaisiswa>")]
pub async fn create_nilai( mut db: Connection<DbConn>, new_nilaisiswa: Json<NewNilaiSiswa>) -> Result<Custom<Value>,String> {
    NilaiSiswaRepository::create(&mut db, new_nilaisiswa.into_inner()).await
        .map(|nilaisiswa| Custom(Status::Created, json!(nilaisiswa)))
        .map_err(|err| format!("Error reading file: {}", err))
}

#[rocket::put("/nilai-siswa/<id>", format="json", data="<nilaisiswa>")]
pub async fn update_nilai( mut db: Connection<DbConn>, id:i32, nilaisiswa: Json<NilaiSiswa>) -> Result<Value,Custom<Value>> {
    NilaiSiswaRepository::update(&mut db, id, nilaisiswa.into_inner()).await
        .map(|nilaisiswa| json!(nilaisiswa))
        .map_err(|e| server_error(e.into())) 
}

#[rocket::delete("/nilai-siswa/<id>")]
pub async fn delete_nilai( mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    NilaiSiswaRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}