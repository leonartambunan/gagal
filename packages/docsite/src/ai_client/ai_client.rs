use std::str::FromStr;

use reqwest::{header::{HeaderValue, CONTENT_TYPE}, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;


#[derive(Debug, Serialize, Deserialize)]
struct AiRequest {
    doc_type: String,
    image_base64: String,
}

pub const API_URL: &str = "https://www.gagal.works/extract";

pub async fn ask_ai(doc_type: &str, content: String) -> Value {

    let new_prompt = AiRequest {
        doc_type: String::from(doc_type),
        image_base64: content
    };

    // let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert("accept", "application/json".parse().unwrap());
    //headers.insert("Accept", "application/vnd.github.v3+json")
    // headers.insert("content-type", " application/json; charset=utf-8".parse().unwrap());
    // headers.insert("X-aaa", "aaa".parse().unwrap());
    // headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));


    let client = reqwest::Client::builder()
    .build()
    .unwrap();

    // let client: Client = Client::builder()
    //     // .timeout(Duration::from_secs(200)) // Set a 5-second timeout
    //     .read_timeout(Duration::from_secs(200))
    //     .build()
    //     .unwrap();

    let resp  = client.post(API_URL)    
    // .headers(headers)  
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .json(&new_prompt)
    // .headers(headers)    
    .fetch_mode_no_cors()
    .send()
    .await;

    println!("r: {resp:?}");

    let ai_final_result = match resp {
        Ok(r) => {
            if r.status().is_success() {
                let txt = match r.text().await {
                    Ok(data) =>  {
                        println!("{data:?}");
                        Value::from_str(data.as_str())
                    },
                    Err(e) => {
                        println!("{e:?}");
                        Value::from_str("{\"error\": \"The downstream AI is refusing request.\"}")
                    }
                };
                txt
           } else {
                println!("didn't get OK status: {}", r.status());
                // "Something error under the streams".to_string()
                Value::from_str("{\"error\": \"Something error under the streams\"}")
           }
    
        },
        Err(e) => {
            println!("{e:?}");
            //format!("The downstream OCR is refusing request. {}", e.to_string())
            Value::from_str("{\"error\": \"The downstream OCR is refusing request\"")
        },
    };

    ai_final_result.unwrap()
}