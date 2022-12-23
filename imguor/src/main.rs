use std::path::Path;
use std::process;
use std::time::{UNIX_EPOCH, Duration};
use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use reqwest::blocking::{Client, multipart::Form};
use serde_json::Value;
struct Imgur<'a> {
    client_id: String,
    image: &'a str
}

impl Imgur<'_> {
    fn upload(&mut self) -> String {
        let mut headers = HeaderMap::new();
        let mut bearer = "Bearer ".to_owned();
        let id_value: String = self.client_id.to_owned();

        bearer.push_str(&id_value);
        
        headers.insert("Authorization", "Bearer 313baf0c7b4d3ff".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        let form_part = Form::new()
        .file("image", self.image)
        .unwrap();

        let client = Client::builder()
        .build()
        .unwrap();

        let resp: String = client.post("https://api.imgur.com/3/upload")
        .headers(headers)
        .multipart(form_part)
        .send()
        .unwrap()
        .text()
        .unwrap();

        resp
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || (args.len() == 2 && args[1] == "--help") {
        println!(
"Usage: imguor [option] [file]
Example: imguor example.png

 --help (this screen)
");
        process::exit(0);
    }

    if !Path::new(&args[1]).exists() {
        println!("error: {} file not found", args[1]);
        process::exit(0);
    }

    let mut imgur_auth = Imgur{ client_id: "313baf0c7b4d3ff".to_string(), image: &args[1] };
    let resp = imgur_auth.upload();

    let json: Value = serde_json::from_str(&resp).unwrap();
    
    // JSON array indexing in Rust is pretty cool
    let status = &json["status"];
    let id = json["data"]["id"].as_str().unwrap();
    let date = UNIX_EPOCH + Duration::from_secs(json["data"]["datetime"].as_u64().unwrap());
    
    let dt = DateTime::<Utc>::from(date);
    let dt_str = dt.format("%Y-%m-%d").to_string();
    let tm_str = dt.format("%H:%M:%S").to_string();

    let delete_hash = json["data"]["deletehash"].as_str().unwrap();
    let width = &json["data"]["width"];
    let height = &json["data"]["height"];
    let link = json["data"]["link"].as_str().unwrap();
    let mp4 = json["data"]["mp4"].as_str().unwrap();

    if status != 200 {
        println!("Something went wrong, error code: {}", status);
    } else {
        if mp4 == "" {
            println!("ID: {id}\nDate: {dt_str}\nTime: {tm_str}\nDelete hash: {delete_hash}\nSize: {width}x{height}\nLink: {link}");
        } else {
            println!("ID: {id}\nDate: {dt_str}\nTime: {tm_str}\nDelete hash: {delete_hash}\nSize: {width}x{height}\nLink: {link}\nMP4: {mp4}");
        }
    }
}
