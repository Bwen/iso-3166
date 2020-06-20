extern crate iso_3166;

const GEONAMES_URL: &str = "https://download.geonames.org/export/dump";
const GEONAMES_FILENAME_COUNTRY: &str = "countryInfo.txt";
const GEONAMES_FILENAME_LANGUAGE: &str = "iso-languagecodes.txt";

use surf;
use structopt::StructOpt;
use async_std::task;
use std::{fs, env};
use std::path::Path;
use async_std::prelude::*;

/// Downloads the required Geonames files for the library info features
/// This is of course optional, you may download the files manually directly from their site:
/// https://download.geonames.org/export/dump/
/// country::Info -> countryInfo.txt
/// language::Info -> iso-languagecodes.txt
///
#[derive(StructOpt, Debug)]
#[structopt(name = "download-files")]
struct Opts {}

fn main() {
    let env_path = env::var("GEONAMES_DIR").expect("Undefined environment variable GEONAMES_DIR");
    let path = Path::new(env_path.as_str());
    if !path.exists() {
        panic!(format!("The GEONAMES_DIR does not exists: {}", env_path))
    }

    // TODO: add flag to update current files
    let mut to_download: Vec<&str> = vec![];
    let files: Vec<&str> = vec![GEONAMES_FILENAME_COUNTRY, GEONAMES_FILENAME_LANGUAGE];
    for file in files {
        let file_path = format!("{}/{}", env_path, file);
        let path = Path::new(file_path.as_str());
        if !path.exists() {
            to_download.push(file);
        }
    }

    if to_download.is_empty() {
        println!("Nothing to download...");
        return;
    }

    task::block_on(async {
        for file in to_download {
            let file_url = format!("{}/{}", GEONAMES_URL, file);
            let response = surf::get(&file_url).recv_string().await.unwrap();
            fs::write(format!("{}/{}", env_path, file), response);
            println!("File {} downloaded successfully", file_url);
        }
    });
}
