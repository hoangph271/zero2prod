mod test_utils;

#[tokio::test]
async fn health_check_works() {
    let endpoint = test_utils::spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", endpoint))
        .send()
        .await
        .expect("Failed to execute request.");

    println!("Status: {:?}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
