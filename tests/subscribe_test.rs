mod common;
use common::app;

#[actix_web::test] 
async fn subscribe_returns_a_200_valid_data(){
    //init 
    let apps = app::spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=goon%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &apps.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions WHERE email=\'goon@gmail.com\'",)
        .fetch_one(&apps.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "goon@gmail.com");
    assert_eq!(saved.name, "le guin");
}


#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let apps = app::spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &apps.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400,response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}.",error_message);
    }
}