
/*use reqwest;

// tokio let's us use "async" on our main function

async fn main() {
    let mut note = String::new();
    
    // chaining .await will yield our query result
    let result = reqwest::get("http://127.0.0.1:5500").await;
    println!("{:?}", result);
}*/
use reqwest::Client;
#[tokio::main]
async fn main() {
  let res = Client::new()
      .get("http://127.0.0.1:5500/javascript/hello.html")
      .send()
      .await
      .expect("failed to get response")
      .text()
      .await
      .expect("failed to get payload");

  println!("{}", res);
}


