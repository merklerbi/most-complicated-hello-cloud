use std::collections::HashMap;

use error_chain::error_chain;
use serde::{Serialize, Deserialize};

use reqwest::header::CONTENT_TYPE;

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let res = client.get("http://worldtimeapi.org/api/timezone/Europe/Berlin")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<GETAPIResponse>()
        .await?;
    
    Ok(())
}