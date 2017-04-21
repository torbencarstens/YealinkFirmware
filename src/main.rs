extern crate clap;
extern crate hyper;
extern crate pretty_bytes;
extern crate regex;
extern crate time;

use clap::{App, Arg};

use hyper::Client;

use pretty_bytes::converter::convert;

use regex::Regex;

use std::env::home_dir;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::io::{Read, Write};
use std::ops::Sub;

fn main() {
    let mut app: App = App::new("YealinkFirmware");
    app = app.arg(Arg::with_name("Target directory").long("directory").short("d").takes_value(true).default_value(".").max_values(1).help("Directory where the zip file will be written to."));
    app = app.arg(Arg::with_name("Remove zip").long("remove").short("r").takes_value(false).help("Deletes the zip file after unzipping."));

    let start = time::now();
    let base_url = "http://support.yealink.com/documentFront/forwardToDocumentDetailPage?documentId=";
    let device_id = 33;
    let url = format!("{}{}", base_url, device_id);

    // Read the content from the support site of the device
    let client = Client::new();
    let mut response = client.get(url.as_str()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    let mut regex = Regex::new("<a href=\"(?P<link>.*\\.zip)\".*\\n\\s*<span class=\"firm-new").expect("Failed to compile new firmware regex.");
    let mut regex_match: Option<regex::Match> = regex.find(body.as_str());

    if regex_match.is_none() {
        println!("Failed to get link via `New` tag, using alternative method.");
        // Get the first link(a) with the parent <div class="file-title"
        let firmware_notes_string = "<div id=\"frnotes\"";
        let start_index = body.as_str().find(firmware_notes_string).expect(format!("Couldn't find firmware notes on `{}`", url).as_str());

        // Firmware name has to be of the following form:
        // \w+\d+-\d+(\.\d+)+\.zip -> e.g. T23-44.81.0.70.zip
        regex = Regex::new("href=\"(?P<link>.*\\w+\\d+-\\d+(\\.\\d+)+\\.zip)\"").expect("Regex failed");
        regex_match = regex.find_at(body.as_str(), start_index);
    }

    let mut captures: Option<regex::Captures> = None;
    if regex_match.is_some() {
        captures = regex.captures(regex_match.unwrap().as_str());
        if captures.is_none() {
            println!("No valid link was found on `{}`.", url)
        }
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
            let mut file = File::create(&path).unwrap();
            let start = time::now();
            match file.write(file_content.as_slice()) {
                Ok(size) => {
                    let end = time::now().sub(start);
                    println!("Successfully wrote `{}` with {} from `{}` in {}.{}s.", path.as_path().to_str().unwrap(), convert(size as f64), link, end.num_seconds(), end.num_milliseconds())
                }
                Err(e) => println!("Writing file `{}` failed due to error: {:?}", filename, e)
            };

            let output_path = match path.parent() {
                Some(val) => { val.to_str().unwrap() }
                None => home_dir.to_str().unwrap()
            };
            println!("Unzipping {} to {}", path.to_str().unwrap(), output_path);
            let start = time::now();
            Command::new("unzip")
                .args(&["-n", path.to_str().unwrap(), "-d", output_path])
                .status()
                .expect("Failed to unzip archive.");
            let end = time::now().sub(start);
            println!("Finished unzipping in {}.{}s", end.num_seconds(), end.num_milliseconds());
        }
    }

    let end = time::now().sub(start);
    println!("Finished execution in {}.{}s", end.num_seconds(), end.num_milliseconds());
}
