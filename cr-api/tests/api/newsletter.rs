// tests/api/newsletter.rs

use crate::helpers::{spawn_app, TestApp, ConfirmationLinks};
use cr_api_docker::routes::confirm;
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
  // Arrange
  let app = spawn_app().await;
  create_unconfirmed_subscriber(&app).await;

  Mock::given(any())
    .respond_with(ResponseTemplate::new(200))
    // We assert that no request is fired at Postmark!
    .expect(0)
    .mount(&app.email_server)
    .await;

  // Act

  // A sketch of the newsletter payload structure.
  // We might change it later on.
  let newsletter_request_body = serde_json::json!({
    "title": "Newsletter title",
    "content": {
      "text": "Newsletter body as plain text",
      "html": "<p>Newsletter body as HTML</p>",
    }
  });

  let response = reqwest::Client::new()
    .post(&format!("{}/newsletters", &app.address))
    .json(&newsletter_request_body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert_eq!(response.status().as_u16(), 200);
  // Mock verifies on Drop that we haven't sent the newsletter email
}

// Use the public API of the application under test to create
// an unconfirmed subscriber

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
  let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

  let _mock_guard = Mock::given(path("/email"))
    .and(method("POST"))
    .respond_with(ResponseTemplate::new(200))
    .named("Create unconfirmed subscriber")
    .expect(1)
    .mount_as_scoped(&app.email_server)
    .await;
  app.post_subscriptions(body.into())
    .await
    .error_for_status()
    .unwrap();

  // We now inspect the requests received by the mock Postmark server
  // to retrieve the confirmation link and return it
  let email_request = &app
      .email_server
      .received_requests()
      .await
      .unwrap()
      .pop()
      .unwrap();
  app.get_confirmation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
  // We can then reuse the same helper and just add
  // an extra step to actually call the confirmation link!
  let confirmation_link = create_unconfirmed_subscriber(app).await;
  reqwest::get(confirmation_link.html)
      .await
      .unwrap()
      .error_for_status()
      .unwrap();
}

