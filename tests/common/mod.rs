use std::process::Command;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::StatusCode;
use reqwest::header;

use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn created_test_siswa(client: &Client) -> Value {
    let response = client.post(format!("{}/siswa", APP_HOST))
        .json(&json!({
            "name": "Testing",
            "kelas": "A"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

   response.json().unwrap()
}

pub fn created_nilai_siswa(client: &Client, siswa: &Value) -> Value {
    let response = client.post(format!("{}/nilai-siswa", APP_HOST))
        .json(&json!({
            "datasiswa_id": siswa["id"], 
            "name":"Testing",
            "nilai":"75",
            "matapelajaran":"Matematika",
            "description":"Tingkatkan Lagi"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

   response.json().unwrap()
}

pub fn delete_test_siswa(client: &Client, datasiswa: Value) {
    let response = client.delete(format!("{}/siswa/{}", APP_HOST,  datasiswa["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}

pub fn delete_test_nilai_siswa(client: &Client, nilaisiswa: Value) {
    let response = client.delete(format!("{}/nilai-siswa/{}", APP_HOST,  nilaisiswa["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}

pub fn get_logged_in_client(username: &str, role: &str) -> Client {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg("1234")
        .arg(role)
        .output()
        .unwrap();

    let client = Client::new();
    let response = client.post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "password": "1234",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap()
    );

    ClientBuilder::new().default_headers(headers).build().unwrap()
}

pub fn get_client_with_logged_in_viewer() -> Client {
    get_logged_in_client("test_viewer", "viewer")
}

pub fn get_client_with_logged_in_admin() -> Client {
    get_logged_in_client("test_admin", "admin")
}

