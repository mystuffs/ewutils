use std::path::Path;
use std::process;
use reqwest::header::HeaderMap;
use reqwest::blocking::{Client, multipart::Form};
use reqwest::redirect::Policy;
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
        .redirect(Policy::none())
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("imguor usage\n-------------\nimguor --upload [image_file] (to upload image)\nimguor --help (this screen)");
        process::exit(0);
    }

    if !Path::new(&args[2]).exists() {
        println!("error: {} file not found", args[2]);
        process::exit(0);
    }

    let mut imgur_auth = Imgur{ client_id: "313baf0c7b4d3ff".to_string(), image: &args[2] };
    let resp = imgur_auth.upload();

    // This structure is ugly, but nothing I can do for now except extracting json directly
    let json: Value = serde_json::from_str(&resp).unwrap();
    let status: Option<&Value> = json.get("status");
    let id: Option<&Value> = json.get("data")
    .and_then(|v| v.get("id"));
    let delete_hash: Option<&Value> = json.get("data")
    .and_then(|v| v.get("deletehash"));
    let width: Option<&Value> = json.get("data")
    .and_then(|v| v.get("width"));
    let height: Option<&Value> = json.get("data")
    .and_then(|v| v.get("height"));
    let link: Option<&Value> = json.get("data")
    .and_then(|v| v.get("link"));
    let mp4: Option<&Value> = json.get("data")
    .and_then(|v| v.get("mp4"));

    if status.unwrap() != 200 {
        println!("Something went wrong, error code: {}", status.unwrap());
    } else {
        if mp4.unwrap() == "" {
            println!("ID: {:0?}\nDelete hash: {1}\nSize: {2}x{3}\nLink: {4}",
            id.unwrap(),
            delete_hash.unwrap(),
            width.unwrap(),
            height.unwrap(),
            link.unwrap());
        } else {
            println!("ID: {:0?}\nDelete hash: {1}\nSize: {2}x{3}\nLink: {4}\nMP4: {5}",
            id.unwrap(),
            delete_hash.unwrap(),
            width.unwrap(),
            height.unwrap(),
            link.unwrap(),
            mp4.unwrap());
        }
    }
    Ok(())
}
