extern crate clap;
extern crate hyper;
extern crate pretty_bytes;
extern crate regex;
extern crate time;

mod archive;

use archive::Zip;

use clap::{App, Arg, ArgMatches};

use hyper::Client;

use pretty_bytes::converter::convert;

use regex::Regex;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::{Read, Write};
use std::ops::Sub;

fn main() {
    let start = time::now();

    let app = get_command_line_app();
    let matches = app.get_matches();

    let base_url = "http://support.yealink.com/documentFront/forwardToDocumentDetailPage?documentId=";
    let device_id = matches.value_of("Device ID").unwrap();
    let url = get_device_url(base_url, device_id);

    let target_directory = get_target_directory(&matches);
    let remove_zip = matches.is_present("Remove zip");

    let client = get_client();
    let body: String = get_body(url.as_str(), &client);

    let new_firmware_regex = "<a href=\"(?P<link>.*\\.zip)\".*\\n\\s*<span class=\"firm-new";
    let new_firmware_regex = Regex::new(new_firmware_regex)
        .expect("Failed to compile new firmware regex.");

    let captures = match get_firmware_match(&new_firmware_regex, body.as_str(), url.as_str()) {
        (val, false) => { get_captures(&new_firmware_regex, val) }
        (val, true) => {
            let alternate_firmware_regex = Regex::new("href=\"(?P<link>.*\\.zip)\"")
                .expect("Failed to compile alternative firmware regex");
            get_captures(&alternate_firmware_regex, val)
        }
    };

    let link = get_link(captures);
    if link.is_none() {
        println!("No link found");
        return;
    }

    let link = link.unwrap();
    let file_content = download_firmware(&link, &client);

    let filename = get_filename_for_firmware(link.as_str());
    let path = get_path(&target_directory, filename);
    let mut file = File::create(&path).unwrap();

    let write_start = time::now();
    match write_file(&mut file, file_content) {
        Ok(size) => {
            let end = time::now().sub(write_start);
            let path = path.as_path().to_str().unwrap();
            let time = format!("{}.{}s", end.num_seconds(), end.num_milliseconds());
            let size = convert(size as f64);
            println!("Successfully wrote `{}` with {} from `{}` in {}.", path, size, link, time)
        }
        Err(e) => {
            println!("Writing file `{}` failed due to error: {:?}", filename, e)
        }
    }

    let output_path = path.parent().unwrap().to_str().unwrap();
    println!("Unzipping {} to {}", path.to_str().unwrap(), output_path);
    let unzip_start = time::now();
    match unzip(&path, output_path, Some(remove_zip)) {
        true => {
            let end = time::now().sub(unzip_start);
            println!("Finished unzipping in {}.{}s", end.num_seconds(), end.num_milliseconds());
        }
        false => {
            println!("Failed unzipping file ({}) due to error", path.to_str().unwrap());
        }
    };

    let end = time::now().sub(start);
    println!("Finished execution in {}.{}s", end.num_seconds(), end.num_milliseconds());
}

fn unzip<'a>(path: &Path, output_path: &'a str, remove: Option<bool>) -> bool {
    Zip::from(path).unzip(Some(output_path.to_owned()), remove).success()
}

fn get_filename_for_firmware(link: &str) -> &str {
    link.rsplit("%2F").nth(0).unwrap()
}

fn get_path<'a>(target_directory: &String, filename: &'a str) -> PathBuf {
    Path::new(&target_directory).join(filename)
}

fn get_firmware_match<'a>(new_firmware_regex: &Regex, body: &'a str, url: &'a str) -> (Option<regex::Match<'a>>, bool) {
    match find_new_firmware(&new_firmware_regex, body) {
        Some(val) => { (Some(val), false) }
        None => (get_first_firmware(body, url), true)
    }
}

fn download_firmware<'a>(link: &'a str, client: &Client) -> Vec<u8> {
    let mut response: hyper::client::Response = match get_response(link, client) {
        Some(val) => { val }
        None => panic!("Error while creating connection to {}", link)
    };

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

    file_content
}

fn get_response<'a>(link: &'a str, client: &Client) -> Option<hyper::client::Response> {
    match client.get(link).send() {
        Ok(val) => { Some(val) }
        Err(e) => {
            println!("Couldn't read data from `{:?}` due to an error: {:?}", link, e);
            None
        }
    }
}

fn write_file(file: &mut File, content: Vec<u8>) -> io::Result<usize> {
    file.write(content.as_slice())
}

fn find_new_firmware<'a>(new_firmware_regex: &Regex, body: &'a str) -> Option<regex::Match<'a>> {
    new_firmware_regex.find(body)
}

fn get_link(captures: Option<regex::Captures>) -> Option<String> {
    match captures {
        Some(captures) => { Some(captures.name("link").unwrap().as_str().to_string()) }
        None => {
            println!("No captures found while searching for link.");
            None
        }
    }
}

fn get_captures<'a>(regex: &Regex, regex_match: Option<regex::Match<'a>>) -> Option<regex::Captures<'a>> {
    match regex_match {
        Some(val) => {
            let searchable: &'a str = val.as_str();
            let captures: Option<regex::Captures<'a>> = regex.captures(searchable);
            captures
        }
        None => {
            println!("No match found while getting captures.");
            None
        }
    }
}

fn get_first_firmware<'a>(body: &'a str, url: &'a str) -> Option<regex::Match<'a>> {
    println!("Failed to get link via `New` tag, using alternative method.");
    // Get the first link(a) with the parent <div class="file-title"
    let firmware_notes_string = "<div id=\"frnotes\"";
    let start_index = body.find(firmware_notes_string).expect(format!("Couldn't find firmware notes on `{}`", url).as_str());

    // Firmware name has to be of the following form:
    // \w+\d+-\d+(\.\d+)+\.zip -> e.g. T23-44.81.0.70.zip
    let regex = Regex::new("href=\"(?P<link>.*(\\w+\\d+-)?\\d+(\\.\\d+)+\\.zip)\"").expect("Regex failed");
    regex.find_at(body, start_index)
}

fn get_device_url(base: &str, id: &str) -> String {
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
    app = app.arg(Arg::with_name("Remove zip").long("remove").short("r").takes_value(true).help("Deletes the zip file after unzipping."));
    app.arg(Arg::with_name("Device ID").long("device-id").short("i").takes_value(true).max_values(1).default_value("33"))
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
