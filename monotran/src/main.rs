use std::collections::HashMap;

use error_chain::error_chain;
use serde::{Serialize, Deserialize};

use reqwest::header::{CONTENT_TYPE, ORIGIN, HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_ENCODING, CACHE_CONTROL, CONNECTION, HOST, UPGRADE, UPGRADE_INSECURE_REQUESTS, ACCEPT_LANGUAGE};

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

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ORIGIN, HeaderValue::from_static("null"));
    // headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    // headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    // headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"));
    // headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    // headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    // headers.insert(HOST, HeaderValue::from_static("worldtimeapi.org"));
    // headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    // headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36"));

    let client = reqwest::Client::new();
    let res = client.get("https://timeapi.io/api/Time/current/zone")
        .headers(headers)
        .send()
        .await?
        .json::<GETAPIResponse>()
        .await?;

    println!("{:?}", res);





    Ok(())
}