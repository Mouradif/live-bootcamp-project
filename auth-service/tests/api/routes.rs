use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn root_signup_returns_200() {
    let app = TestApp::new().await;

    let response = app.signup().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn root_login_returns_200() {
    let app = TestApp::new().await;

    let response = app.login().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn root_logout_returns_200() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn root_verify_mfa_returns_200() {
    let app = TestApp::new().await;

    let response = app.verify_mfa().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn root_verify_token_returns_200() {
    let app = TestApp::new().await;

    let response = app.verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
}
