use reqwest::Url;
use http::Uri;
use std::borrow::Cow;

fn get_params_by_url(req: String) -> Vec<(String, String)> {
  let uri = req.parse::<Uri>().unwrap();

  let tmp = Url::parse(&uri.to_string()).unwrap();
  let params = tmp.query_pairs();

  let vec_params = params.collect::<Vec<(Cow<'_, str>, Cow<'_, str>)>>();

  let mut res = vec![];
  for vp in vec_params {
    res.push((String::from(vp.0), String::from(vp.1)));
  }
  
  return res;
}

#[tokio::main]
async fn main() {
  /*let params = [("foo", "kjj"), ("baz", "quux")];
  let resp = Client::new()
    .post("http://127.0.0.1:5500/test.html?foo=1&bar=2")
    .form(&params)
    .body("the exact body that is sent")
    .send()
    .await;*/

  let request = String::from("http://127.0.0.1:5500/test.html?foo=value1&bar=value2");

  let resp = get_params_by_url(request);

  dbg!(resp);
}