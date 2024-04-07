use reqwest::StatusCode;
use rocket::serde::json::{serde_json::json, Value};

pub mod common;


#[test]
fn test_get_nilai() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);
    let a_nilaisiswa = common::created_nilai_siswa(&client, &siswa);
    let b_nilaisiswa = common::created_nilai_siswa(&client, &siswa);

    // Test
    let response = client.get(format!("{}/nilai-siswa", common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_nilaisiswa));
    assert!(json.as_array().unwrap().contains(&b_nilaisiswa));

    // Cleanup
    common::delete_test_nilai_siswa(&client, a_nilaisiswa);
    common::delete_test_nilai_siswa(&client, b_nilaisiswa);
    common::delete_test_siswa(&client, siswa);
}

#[test]
fn test_create_nilai_siswa() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);
    
    // Test
    let response = client.post(format!("{}/nilai-siswa", common::APP_HOST))
        .json(&json!({
            "datasiswa_id": siswa["id"], 
            "name":"Testing",
            "nilai":"75",
            "matapelajaran": "Matematika",
            "description": "Tingkatkan Lagi"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_nilaisiswa: Value = response.json().unwrap();
    assert_eq!(a_nilaisiswa, json!({
        "id": a_nilaisiswa["id"],
        "datasiswa_id": siswa["id"],
        "name":"Testing",
        "nilai":"75",
        "matapelajaran": "Matematika",
        "description": "Tingkatkan Lagi",
        "created_at": a_nilaisiswa["created_at"],
    })); 

    // Cleanup
    common::delete_test_nilai_siswa(&client, a_nilaisiswa );
    common::delete_test_siswa(&client, siswa);
       
}

#[test]
fn test_view_nilai() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);
    let a_nilaisiswa = common::created_nilai_siswa(&client, &siswa);

    // Test
    let response = client.get(format!("{}/nilai-siswa/{}", common::APP_HOST, a_nilaisiswa["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_nilaisiswa: Value = response.json().unwrap();
    assert_eq!(a_nilaisiswa, json!({
        "id": a_nilaisiswa["id"],
        "name":"Testing",
        "nilai":"75",
        "matapelajaran": "Matematika",
        "description": "Tingkatkan Lagi",
        "datasiswa_id": siswa["id"],
        "created_at": a_nilaisiswa["created_at"],
    }));

    // Cleanup
    common::delete_test_nilai_siswa(&client, a_nilaisiswa);
    common::delete_test_siswa(&client, siswa);
}


#[test]
fn test_update_nilai_siswa() {
    // Setup 
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);
    let a_nilaisiswa = common::created_nilai_siswa(&client, &siswa);

    // Test 
    
    let response = client.put(format!("{}/nilai-siswa/{}", common::APP_HOST,  a_nilaisiswa["id"]))
        .json(&json!({ 
            "name":"Testing",
            "nilai":"95",
            "matapelajaran": "IPS",
            "description": "Pertahankan",
            "datasiswa_id": siswa["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_nilaisiswa: Value = response.json().unwrap();
    assert_eq!(a_nilaisiswa, json!({
        "id": a_nilaisiswa["id"], 
        "name":"Testing",
        "nilai":"95",
        "matapelajaran": "IPS",
        "description": "Pertahankan",
        "datasiswa_id": siswa["id"],
        "created_at": a_nilaisiswa["created_at"],
    }));

    // Cleanup
    common::delete_test_nilai_siswa(&client, a_nilaisiswa);
    common::delete_test_siswa(&client, siswa);

}

#[test]
fn test_delete_nilai_siswa() {
    // Setup 
    let client = common::get_client_with_logged_in_admin();
    let siswa = common::created_test_siswa(&client);
    let a_nilaisiswa = common::created_nilai_siswa(&client, &siswa);

    // Test
    let response = client.delete(format!("{}/nilai-siswa/{}", common::APP_HOST, a_nilaisiswa["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Cleanup
    common::delete_test_siswa(&client, siswa);

}
