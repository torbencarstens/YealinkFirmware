extern crate clap;
extern crate hyper;
extern crate pretty_bytes;
extern crate regex;
extern crate time;

use clap::{App, Arg, ArgMatches};

use hyper::Client;

use pretty_bytes::converter::convert;

use regex::Regex;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::io::{Read, Write};
use std::ops::Sub;

fn main() {
    let start = time::now();

    let app = get_command_line_app();

    let base_url = "http://support.yealink.com/documentFront/forwardToDocumentDetailPage?documentId=";
    let t23p_id = 33;
    let url = get_device_url(base_url, t23p_id);

    // Target directory has a default value -> Safe usage of unwrap
    let matches = app.get_matches();
    let target_directory = get_target_directory(&matches);
    let remove_zip = matches.is_present("Remove zip");

    // Read the content from the support site of the device
    let client = get_client();
    let body: String = get_body(url.as_str(), &client);

    let regex = Regex::new("<a href=\"(?P<link>.*\\.zip)\".*\\n\\s*<span class=\"firm-new").expect("Failed to compile new firmware regex.");
    let regex_match: Option<regex::Match> = match regex.find(body.as_str()) {
        Some(val) => { Some(val) }
        None => get_new_firmware(body.as_str())
    };

    let captures: Option<regex::Captures> = get_captures(&regex, regex_match);

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

            let filename = link.rsplit("%2F").nth(0).unwrap();
            let path = Path::new(&target_directory).join(filename);
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
                None => target_directory.as_str()
            };
            println!("Unzipping {} to {}", path.to_str().unwrap(), output_path);
            let start = time::now();
            Command::new("unzip")
                .args(&["-n", path.to_str().unwrap(), "-d", output_path])
                .status()
                .expect("Failed to unzip archive.");
            let end = time::now().sub(start);
            if remove_zip {
                match fs::remove_file(&path) {
                    Ok(_) => { println!("Successfully deleted .zip file.") }
                    Err(e) => println!("Failed to delete .zip file: {:?}", e)
                };
            }
            println!("Finished unzipping in {}.{}s", end.num_seconds(), end.num_milliseconds());
        }
    }

    let end = time::now().sub(start);
    println!("Finished execution in {}.{}s", end.num_seconds(), end.num_milliseconds());
}

fn get_captures<'a>(regex: &Regex, regex_match: Option<regex::Match<'a>>) -> Option<regex::Captures<'a>> {
    match regex_match {
        Some(val) => {
            let searchable: &'a str = val.as_str();
            let captures: Option<regex::Captures<'a>> = regex.captures(searchable);
            captures
        }
        None => None
    }
}

fn get_new_firmware<'a>(body: &'a str) -> Option<regex::Match<'a>> {
    let url = "test";
    println!("Failed to get link via `New` tag, using alternative method.");
    // Get the first link(a) with the parent <div class="file-title"
    let firmware_notes_string = "<div id=\"frnotes\"";
    let start_index = body.find(firmware_notes_string).expect(format!("Couldn't find firmware notes on `{}`", url).as_str());

    // Firmware name has to be of the following form:
    // \w+\d+-\d+(\.\d+)+\.zip -> e.g. T23-44.81.0.70.zip
    let regex = Regex::new("href=\"(?P<link>.*\\w+\\d+-\\d+(\\.\\d+)+\\.zip)\"").expect("Regex failed");
    regex.find_at(body, start_index)
}

fn get_device_url(base: &str, id: i32) -> String {
    format!("{}{}", base, id)
}

fn get_target_directory<'a>(matches: &ArgMatches<'a>) -> String {
    match matches.value_of("Target directory") {
        Some(val) => { val.to_string() }
        // Target directory should have a default value, JIC
        None => ".".to_string()
    }
}

fn get_command_line_app<'a, 'b>() -> App<'a, 'b> {
    let mut app: App = App::new("YealinkFirmware");
    app = app.arg(Arg::with_name("Target directory").long("directory").short("d").takes_value(true).default_value(".").max_values(1).help("Directory where the zip file will be written to."));
    app.arg(Arg::with_name("Remove zip").long("remove").short("r").takes_value(false).help("Deletes the zip file after unzipping."))
}

fn get_client() -> Client {
    Client::new()
}

fn get_body<'a>(url: &'a str, client: &Client) -> String {
    let mut response = client.get(url).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    body
}
