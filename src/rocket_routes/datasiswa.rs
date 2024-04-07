use crate::models::DataSiswa;
use crate::models::NewDataSiswa;
use crate::models::User;
use crate::repositories::DataSiswaRepositoritory;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::rocket_routes::{DbConn, server_error};

#[rocket::get("/siswa")]
pub async fn get_siswa(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    DataSiswaRepositoritory::find_multiple(&mut db, 100).await
        .map(|datasiswa| json!(datasiswa))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/siswa/<id>")]
pub async fn view_siswa( mut db: Connection<DbConn>, id:i32, _user: User) -> Result<Value,Custom<Value>> {
    DataSiswaRepositoritory::find(&mut db, id).await
        .map(|datasiswa| json!(datasiswa))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/siswa", format="json", data="<new_datasiswa>")]
pub async fn create_siswa( mut db: Connection<DbConn>, new_datasiswa: Json<NewDataSiswa>) -> Result<Custom<Value>,String> {
    DataSiswaRepositoritory::create(&mut db, new_datasiswa.into_inner()).await
        .map(|datasiswa| Custom(Status::Created, json!(datasiswa)))
        .map_err(|err| format!("Error reading file: {}", err))
}

#[rocket::put("/siswa/<id>", format="json", data="<datasiswa>")]
pub async fn update_siswa( mut db: Connection<DbConn>, id:i32, datasiswa: Json<DataSiswa>) -> Result<Value,Custom<Value>> {
    DataSiswaRepositoritory::update(&mut db, id, datasiswa.into_inner()).await
        .map(|datasiswa| json!(datasiswa))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/siswa/<id>")]
pub async fn delete_siswa( mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    DataSiswaRepositoritory::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}