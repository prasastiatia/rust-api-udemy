extern crate rust;

use rocket_db_pools::Database;



#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", rocket::routes![
        rust::rocket_routes::authorization::login,
        rust::rocket_routes::datasiswa::get_siswa,
        rust::rocket_routes::datasiswa::view_siswa,
        rust::rocket_routes::datasiswa::create_siswa,
        rust::rocket_routes::datasiswa::update_siswa,
        rust::rocket_routes::datasiswa::delete_siswa,
        rust::rocket_routes::nilaisiswa::get_nilai,
        rust::rocket_routes::nilaisiswa::view_nilai,
        rust::rocket_routes::nilaisiswa::create_nilai,
        rust::rocket_routes::nilaisiswa::update_nilai,
        rust::rocket_routes::nilaisiswa::delete_nilai,
    ])
    .attach(rust::rocket_routes::CacheConn::init())
    .attach(rust::rocket_routes::DbConn::init())
    .launch()
    .await;
}