use reqwest::{Response};
use serde_json::{self, Value};
use tracing::trace;


/// # post_json_request
/// ___
/// `post_json_request` will simply post the given JSON body and and return a Result
/// with the either a response or an error code. 
pub async fn post_json_request(body: Value, uri: &String) -> Result<Response, String> {
    trace!("Posting JSON request to: {}", uri);

    let json: String = body.to_string();

    let client = reqwest::ClientBuilder::default()
    .build().expect("Failed building reqwest Client!");

    match client.post(uri)
    .body(json)
    .header("Content-Type", "application/json")
    .header("Accept", "*/*")
    .header("Accept-Encoding", "gzip, deflate, br")
    .header("Connection", "keep-alive")
    .send().await {
        Ok(response) => {
            Ok(response)
        },
        Err(e) => {
            Err(format!("Post request error: {}", e.to_string()))
        }
    }    
}