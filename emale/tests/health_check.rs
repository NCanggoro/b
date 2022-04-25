// use emale::main;
use std::net::TcpListener;
use serde::Deserialize;

#[actix_rt::test]
async fn health_check_works() {
  let addr = spawn_app();
  let client = reqwest::Client::new();
  
  
  let res = client
  .get(&format!("{}/health_check", &addr))
  .send()
  .await
    .expect("Request Failed");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
  }
  
  #[actix_rt::test]
  async fn pow_works() {
	
	#[derive(Deserialize)]
	struct ResponseVal {
		num: i32
	}
	
  let addr = spawn_app();
  let client = reqwest::Client::new();


  let res = client
    .get(&format!("{}/pow2/2", &addr))
    .send()
    .await
    .expect("Request Failed");


  assert!(res.status().is_success());
  assert_eq!(4, 
		res
		.json::<ResponseVal>()
		.await
		.expect("Failed")
		.num
	);
}

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let server = emale::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  format!("http://127.0.0.1:{}", port)
}