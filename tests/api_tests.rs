use warp::test::request;
use serverless_rust::api;
use serverless_rust::storage::Storage;
use std::sync::Arc;

#[tokio::test]
async fn test_register_function() {
    let db_path = "test_db_register_function";
    let storage = Arc::new(Storage::init_with_path(db_path).expect("Failed to initialize storage"));

    let body = r#"
    {
        "name": "factorial",
        "code": "(module (func (export \"main\") (param i32) (result i32) local.get 0))"
    }
    "#;

    let response = request()
        .method("POST")
        .path("/register")
        .body(body)
        .reply(&api::server(storage))
        .await;

    assert_eq!(response.status(), 200, "Response: {:?}", response.body());

    // Cleanup
    std::fs::remove_dir_all(db_path).expect("Failed to clean up test database");
}

#[tokio::test]
async fn test_invoke_function() {
    let db_path = "test_db_invoke_function";
    let storage = Arc::new(Storage::init_with_path(db_path).expect("Failed to initialize storage"));

    storage
        .save_function(
            "factorial".to_string(),
            "(module (func (export \"main\") (param i32) (result i32) local.get 0))".to_string(),
        )
        .expect("Failed to save function");

    let body = r#"
    {
        "name": "factorial",
        "input": [5]
    }
    "#;

    let response = request()
        .method("POST")
        .path("/invoke")
        .body(body)
        .reply(&api::server(storage))
        .await;

    assert_eq!(response.status(), 200, "Response: {:?}", response.body());

    let body_str = std::str::from_utf8(response.body()).expect("Invalid UTF-8 in response body");
    assert!(body_str.contains("5")); // Verify the result

    // Cleanup
    std::fs::remove_dir_all(db_path).expect("Failed to clean up test database");
}
