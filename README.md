<h1 align="center">psychonaut_wiki_api</h1>

<div align="center">
    <img alt="GitHub Workflow Status (event)" src="https://img.shields.io/github/workflow/status/diinki/psychonaut_wiki_api/Rust?event=push&style=flat-square"/>
    <img alt="Crates.io" src="https://img.shields.io/crates/v/psychonaut_wiki_api?style=flat-square"/>
    <img alt="GitHub" src="https://img.shields.io/github/license/diinki/psychonaut_wiki_api?style=flat-square">
</div>


This crate/library provides an easy-to-use API to access data from 
[Psychonaut Wiki](https://psychonautwiki.org), a wiki which contains
information about substances.

## Overview (=w=)
This API provides a simple function which returns a data structure containing
all information the wiki has about a substance with a given name.

This API is used in other projects such as terminal_psychonautica, which can
be used as an example of how this API is used; despite that, this is not needed
since it is a very simple API.

## Usage & Examples </>

* Add the Crate to your project

```toml
psychonaut_wiki_api = "0.1.0"
```

* Fetch information for any substance

```rust
use psychonaut_wiki_api::query_substance;

#[tokio::main]
async fn main() {
    let result = query_substance(&"LSD".to_string()).await;

    // The function may return an error, hence the match pattern.
    match result {
        Ok(data) => {
            // Print out data about the substance to the console.
            // Use the data returned in whichever way you please.
            for substance in data.substances.unwrap() {
                println!("Name: {:?}", substance.name);
                println!("Routes Of Administration: {:?}", substance.routes_of_administration);
                println!("Effects: {:?}", substance.effects);
            }  
        }
        Err(error) => {
            // Feel free to contact me about any errors!
            println!("Error: {}", error);
        }
    }
}
```

## @-Some Notes-@
* This crate uses the Tokio async runtime, as making requests in a synchronous manner is slower.
* If a substance is not found, the `substances` vector will simply be None.
* This Crate is an independent project by me.
* This Crate is early, feel free to contribute, use, or give feedback! You can contact me on `diinki@imp.works`.

## Licensing
This crate is licensed under the permissive MIT License.
