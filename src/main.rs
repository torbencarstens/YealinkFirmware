extern crate hyper;
extern crate pretty_bytes;
extern crate regex;
extern crate time;

use hyper::Client;

use pretty_bytes::converter::convert;

use regex::Regex;

use std::env::home_dir;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use std::ops::Sub;

fn main() {
    let start = time::now();
    let base_url = "http://support.yealink.com/documentFront/forwardToDocumentDetailPage?documentId=";
    let device_id = 33;
    let url = format!("{}{}", base_url, device_id);

    // Read the content from the support site of the device
    let client = Client::new();
    let mut response = client.get(url.as_str()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    // Get the first link(a) with the parent <div class="file-title"
    let regex = Regex::new("<div class=\"file-title\">\\n\\s*<a href=\"(?P<link>[^\"]+\\.zip)\"").expect("Regex failed");
    let captures = regex.captures(body.as_str());
    if captures.is_none() {
        println!("No valid link was found on `{}`.", url)
    }

    if captures.is_some() {
        let link = captures.unwrap().name("link").unwrap().as_str();
        let response: Option<hyper::client::Response> = match client.get(link).send() {
            Ok(val) => { Some(val) }
            Err(e) => {
                println!("Couldn't read data from `{:?}` due to an error: {:?}", link, e);
                None
            }
        };

        if response.is_some() {
            let mut response = response.unwrap();
            let mut file_content = Vec::new();

            let start = time::now();
            match response.read_to_end(&mut file_content) {
                Ok(size) => {
                    let end = time::now().sub(start);
                    let time = format!("{}.{:.2}s", end.num_seconds(), end.num_milliseconds());
                    println!("Successfully read binary data from `{}` with size {} in {}", link, convert(size as f64), time)
                }
                Err(error) => println!("Downloading firmware from `{}` failed due to error: {:?}", link, error)
            };

            let home_dir = home_dir().unwrap();
            let filename = link.rsplit("%2F").nth(0).unwrap();
            let path = Path::new(&home_dir).join(filename);
            let mut file = File::create(path).unwrap();
            match file.write(file_content.as_slice()) {
                Ok(size) => { println!("Successfully wrote `{}` with {} from `{}`.", filename, convert(size as f64), link) }
                Err(e) => println!("Writing file `{}` failed due to error: {:?}", filename, e)
            };
        }
    }

    let end = time::now().sub(start);
    println!("Finished execution in {}.{}s", end.num_seconds(), end.num_milliseconds());
}
