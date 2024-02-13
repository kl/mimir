use crate::helpers::spawn_app;

#[tokio::test]
async fn new_post_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "url_id=new-post&title=first&markdown=__rad__";

    // Act
    app.login().await;
    let response = app.post("admin/new_post", body).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn new_post_returns_a_401_when_not_authenticated() {
    // Arrange
    let app = spawn_app().await;
    let body = "url_id=new-post&title=first&markdown=__rad__";

    // Act
    let response = app.post("admin/new_post", body).await;

    // Assert
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn new_post_persists_the_new_blog_post() {
    // Arrange
    let app = spawn_app().await;
    let _body =

    // Act
    app.login().await;
    app.post(
        "admin/new_post",
        "url_id=new-post&title=first&markdown=__rad__",
    )
    .await;

    let posts = app
        .admin_use_case
        .get_all_posts()
        .await
        .expect("Error fetching blog posts");

    assert_eq!(posts.len(), 1, "expected only one blog post");
    assert_eq!(posts[0].markdown, "__rad__");
}

#[tokio::test]
async fn viewing_non_existing_post_returns_404() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let resp = app.get("blog/random-id-that-does-not-exist").await;

    // Assert
    assert_eq!(resp.status().as_u16(), 404);
}

#[tokio::test]
async fn viewing_an_existing_post_returns_200() {
    // Arrange
    let app = spawn_app().await;
    let body = "url_id=new-post&title=first&markdown=__rad__";

    // Act
    app.login().await;
    app.post("admin/new_post", body).await;
    let resp = app.get("blog/new-post").await;

    // Assert
    assert_eq!(resp.status().as_u16(), 200);
}

#[tokio::test]
async fn test_error_message_information() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let err_resp = reqwest::Client::new()
        .get(&format!("{}/blow_up", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(err_resp.status().as_u16(), 500);
}
