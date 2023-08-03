use zero2prod::run;
use reqwest;


async fn spawn_app()  {
    let server=zero2prod::run().expect("error spawning server");
    let _=tokio::spawn(server);
}


#[tokio::test]
async fn health_check_works() {
  spawn_app().await.expect("failed to spawn app");
  let client=reqwest::Client::new();
  let response = client.get("http://127.0.0.1:8000/healthcheck")
      .send()
      .await
      .expect("failed to execute request");

  assert!(response.status().is_success() );
  assert_eq!(Some(0),response.content_length());
}



