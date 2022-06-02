use reqwest::{Response};
use serde::Deserialize;
use gql_client::{Client};
use tracing::{trace, error};

use crate::substance::Substance;
use crate::types::ResponseError;


/// # post_json_request
/// ___
/// `post_json_request` will simply post the given JSON body and and return a Result
/// with the either a response or an error code. 
pub async fn post_json_request(body: &String, uri: &String) -> Result<Response, String> {
    trace!("Posting JSON request to: {}", uri);

    let json: String = body.clone();

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

#[derive(Deserialize, Debug)]
pub struct Data {
    pub errors: Option<Vec<ResponseError>>,
    pub substances: Option<Vec<Substance>>
}


pub async fn post_graphql_request(gql_query: &String, uri: &String) -> Result<Data, String>{
    trace!("Posting GraphQL Request to {}", uri);

    let mut result: Data = Data{
        errors: None,
        substances: None
    };

    let client = Client::new(uri);
    let data = client.query::<Data>(gql_query).await;


    match data {
        Ok(data) => {
            trace!("GraphQL Request had a valid data return value.");
            if data.is_some() {
                result = data.expect("Data should be some");
            }
            Ok(result)
        },
        Err(gql_error) => {
            error!("GraphQL Error detected on post: {}", gql_error.message());
            Err(format!("GraphQL Error detected on post: {}", gql_error.message()))
        },
    }

}