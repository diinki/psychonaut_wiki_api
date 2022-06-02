use crate::{http::request::{post_json_request, post_graphql_request, Data}, substance::Substance};



pub async fn query_substance(name: &String) -> Result<Data, String> {
    // I know, this is kinda wonky but I'll clean it up later by putting it in a seperate file.
    let graphql_request = "
    {
        substances(query: \"NAME_OF_SUBSTANCE\") {
            name
    
            roas {
                name
    
                dose {
                    units
                    threshold
                    heavy
                    common { min max }
                    light { min max }
                    strong { min max }
                }
    
                duration {
                    afterglow { min max units }
                    comeup { min max units }
                    duration { min max units }
                    offset { min max units }
                    onset { min max units }
                    peak { min max units }
                    total { min max units }
                }
    
                bioavailability {
                    min max
                }
            }
    
            effects {
                name url
            }
        }
    }
    ";
    let request = graphql_request.replace("NAME_OF_SUBSTANCE", name);
    post_graphql_request(&request, &"https://api.psychonautwiki.org".to_string()).await

}


#[cfg(test)]
mod tests {
use crate::query::query_substance;

    #[test]
    pub fn query_for_substance_test() {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let result = query_substance(&"Psilocin".to_string()).await;
            match result {
                Ok(data) => {
                    match data.substances {
                        Some(substances) => {
                            for substance in substances {
                                println!("------");
                                println!("Found Substance: {:?}\nEffects:\n", substance.name);
                                println!("{:#?}", substance.effects);
                                println!("------");
                            }
                        },
                        None => {
                            println!("No Data response found for substance, still no errors.");
                        },
                    }
                }
                Err(e) => {
                    println!("ERROR: {}", e);
                }
            }
        });
    }
}
