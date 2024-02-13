use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get("health_check").await;

    // Assert
    assert!(response.status().is_success());
}
