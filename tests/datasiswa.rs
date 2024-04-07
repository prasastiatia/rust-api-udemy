
use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

pub mod common;

#[test]
fn test_get_siswa() {
    // Setup 
    let client = common::get_client_with_logged_in_admin();
    let siswa1 = common::created_test_siswa(&client);
    let siswa2 = common::created_test_siswa(&client);

    // Test
    let response = client.get(format!("{}/siswa", common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&siswa1));
    assert!(json.as_array().unwrap().contains(&siswa2));

    // Cleanup
    common::delete_test_siswa(&client, siswa1);
    common::delete_test_siswa(&client, siswa2);
}

#[test]
fn test_view_siswa() {
    // Setup 
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);
    

    // Test
    let response = client.get(format!("{}/siswa/{}", common::APP_HOST, siswa["id"]))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let siswa: Value = response.json().unwrap();
    assert_eq!(siswa, json!({
        "id": siswa["id"],
        "name": "Testing",
        "kelas": "A",
        "created_at": siswa["created_at"],
    }));

    // Cleanup
    common::delete_test_siswa(&client, siswa);
}

#[test]
fn test_create_siswa() {
    // Setup
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/siswa")
        .json(&json!({
            "name":"Testing",
            "kelas":"A"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let datasiswa: Value = response.json().unwrap();
    assert_eq!(datasiswa, json!({
        "id": datasiswa["id"],
        "name":"Testing",
        "kelas":"A",
        "created_at": datasiswa["created_at"],
    })); 
       
}

#[test]
fn test_update_siswa() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);

    // Test
    let response = client.put(format!("{}/siswa/{}", common::APP_HOST, siswa["id"]))
        .json(&json!({
            "name": "Testing 1",
            "kelas": "A"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let datasiswa: Value = response.json().unwrap();
    assert_eq!(datasiswa, json!({
        "id": datasiswa["id"],
        "name":"Testing 1",
        "kelas":"A",
        "created_at": datasiswa["created_at"],
    }));

    // Cleanup
    common::delete_test_siswa(&client, datasiswa)
}

#[test]
fn test_delete_siswa() {
    // Setup 
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);

    
    let response = client.delete(format!("http://127.0.0.1:8000/siswa/{}", siswa["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}