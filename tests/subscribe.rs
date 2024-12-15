mod test_utils;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let api_root = test_utils::spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin@gmail.com";

    let response = client
        .post(format!("{}/subscriptions", api_root))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request to POST /subscriptions");

    assert!(response.status().is_success());
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let api_root = test_utils::spawn_app();
    let client = reqwest::Client::new();

    // let body = "name=&email=ursula_le_guin&email=";
    let test_cases = vec![
        ("name=le%20guin", "payload missing email"),
        ("email=ursula_le_guin@gmail.com", "payload missing name"),
        ("", "payload missing email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", api_root))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request to POST /subscriptions");

        assert!(response.status().is_client_error());
        assert_eq!(
            response.status(),
            reqwest::StatusCode::BAD_REQUEST,
            "API expected to fail with {}",
            error_message
        );
    }
}
