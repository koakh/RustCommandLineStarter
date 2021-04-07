#![allow(dead_code)]
use serde::{de, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;

// updated version of this module working with cryptlex api is here
// ~/Development/@CriticalLinksBitBucket/c3next/c3-cla/cryptlex-cli/src/http_client.rs

// https://rust-lang-nursery.github.io/rust-cookbook/intro.html
// https://tokio.rs/tokio/tutorial/async

// https://crates.io/crates/reqwest
// https://crates.io/crates/serde_json

// Operating on untyped JSON values
// Any valid JSON data can be manipulated in the following recursive enum representation. This data structure is serde_json::Value.
// https://docs.serde.rs/serde_json/#operating-on-untyped-json-values
// A string of JSON data can be parsed into a serde_json::Value by the serde_json::from_str function. There is also from_slice for parsing from a byte slice &[u8] and from_reader for parsing from any io::Read like a File or a TCP stream.

// https://users.rust-lang.org/t/how-to-get-ok-value-return-from-the-function/40848

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HttpBinResponse {
  // must be public
  pub origin: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct HttpResponse {
  status: String,
  headers: [String],
}

/// Operating on untyped JSON values
/// async request to serde_json::Value
pub async fn async_request_untyped() -> Result<Value, Box<dyn std::error::Error> /*reqwest::Error*/>
{
  let res = reqwest::get("http://httpbin.org/get").await.unwrap();
  println!("status: {}", res.status());
  println!("headers:\n{:#?}", res.headers());
  let body = res.text().await.unwrap();
  println!("body:\n{}", body);

  // Parse the string of data into serde_json::Value.
  let data = r#"
  {
      "name": "John Doe",
      "age": 43,
      "phones": [
          "+44 1234567",
          "+44 2345678"
      ]
  }"#;
  let v: Value = serde_json::from_str(data)?;

  // Access parts of the data by indexing with square brackets.
  // output value in caller
  // println!("Please call {} at the number {}", v["name"], v["phones"][0]);

  Ok(v)
}

/// Parsing JSON as strongly typed data structures
// pub async fn async_request() -> Result<HttpBinResponse, Box<dyn std::error::Error>> {
pub async fn async_request_generic_typed<T: de::DeserializeOwned>() -> Result<Box<T>, Box<dyn std::error::Error>> {  
  let res = reqwest::get("http://httpbin.org/get").await.unwrap();
  println!("status: {}", res.status());
  println!("headers:\n{:#?}", res.headers());
  let body = res.text().await.unwrap();
  println!("body:\n{}", body);
  let v: Box<T> = serde_json::from_str(&body)?;
  Ok(v)
}

/// block request to unit
pub fn block_request() -> Result<(), Box<dyn std::error::Error>> {
  let mut res = reqwest::blocking::get("https://httpbin.org/ip")?;
  let mut body = String::new();
  res.read_to_string(&mut body)?;
  println!("{:#?}", res);
  println!("Status: {}", res.status());
  println!("Headers:\n{:#?}", res.headers());
  println!("Body:\n{}", body);
  Ok(())
}

/// block request to hashmap
pub fn block_request_hashmap() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
  let res = reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
  Ok(res)
}
