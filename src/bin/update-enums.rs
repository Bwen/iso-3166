use grep_matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::{Searcher, SearcherBuilder};

use grep_matcher::{Captures, Matcher};
use std::fs;
use std::fs::File;
use std::io::Error;
use structopt::StructOpt;

const GEONAMES_COUNTRYINFO_FILE: &str = "geonames-files/countryInfo.txt";
const ISO_31661_FILE: &str = "src/country/alpha2.rs";
const ISO_31663_FILE: &str = "src/country/alpha3.rs";
const ISO_3166_FIPS_FILE: &str = "src/country/fips.rs";
const ISO_3166_TLD_FILE: &str = "src/country/tld.rs";
const ISO_3166_CURRENCY_FILE: &str = "src/country/currency.rs";
const ISO_3166_CONTINENT_FILE: &str = "src/country/continent.rs";

const GEONAMES_LANGUAGES_FILE: &str = "geonames-files/iso-languagecodes.txt";
const ISO_6391_FILE: &str = "src/language/alpha2.rs";
const ISO_6392_FILE: &str = "src/language/alpha3b.rs";
const ISO_6393_FILE: &str = "src/language/alpha3.rs";

/// Utility to keep the enums up to date with geonames updates
#[derive(StructOpt, Debug)]
#[structopt(name = "update-enums")]
struct Opts {}

fn main() {
    let opts: Opts = Opts::from_args();

    let countries_languages = update_country_enums();
    update_language_enums(countries_languages);
}

fn update_language_enums(countries_languages: Vec<String>) {
    let regex = "(?m)^([a-z]{3})\t(.*?)\t([a-z]{2})\t";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: Vec<Vec<String>> = vec![vec![], vec![], vec![]];
    Searcher::new()
        .search_path(
            &matcher,
            GEONAMES_LANGUAGES_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);

                if captures.get(1).is_some() || captures.get(2).is_some() || captures.get(3).is_some() {
                    let iso6393 = line[captures.get(1).unwrap()].to_uppercase(); // Alpha3
                    let iso6391 = line[captures.get(3).unwrap()].to_uppercase(); // Alpha2
                    let iso6392 = line[captures.get(2).unwrap()].to_uppercase(); // Alpha3b

                    let mut alpha3b = String::new();
                    if iso6392.contains("/") {
                        let parts: Vec<&str> = iso6392.split("/").collect();
                        alpha3b = parts[0].trim().to_string();
                    } else {
                        alpha3b = iso6392.to_string();
                    }

                    // The `countries_languages` does not contain counter parts ex: fr = fra, which we need to include
                    if countries_languages.contains(&iso6391) || countries_languages.contains(&iso6393) || countries_languages.contains(&alpha3b) {
                        if !iso6393.is_empty() {
                            matches[0].push(iso6393);
                        }

                        if !iso6391.is_empty() {
                            matches[2].push(iso6391);
                        }

                        if !alpha3b.is_empty() {
                            matches[1].push(alpha3b);
                        }
                    }
                }

                Ok(true)
            }),
        )
        .unwrap();

    for results in &mut matches {
        results.sort();
        results.dedup();
    }

    update_enum_file(&matches[0], ISO_6393_FILE);
    update_enum_file(&matches[1], ISO_6392_FILE);
    update_enum_file(&matches[2], ISO_6391_FILE);
}

fn update_country_enums() -> Vec<String> {
    let mut languages: Vec<String> = vec![];
    let regex = "(?m)^([A-Z]{2})\t([A-Z]{3})\t([0-9]{1,3})\t([A-Z]{0,2})\t.*?\t.*?\t.*?\t.*?\t([A-Z]{2})\t(.[a-z]{2,4})\t([A-Z]{3})\t.*?\t.*?\t.*?\t.*?\t([a-zA-Z,-]+)\t([0-9]+)\t([A-Z,]+)\t";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];
    Searcher::new()
        .search_path(
            &matcher,
            GEONAMES_COUNTRYINFO_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);
                matches[0].push(line[captures.get(1).unwrap()].to_string()); // Alpha2
                matches[1].push(line[captures.get(2).unwrap()].to_string()); // Alpha3
                matches[2].push(line[captures.get(4).unwrap()].to_string()); // Fips
                matches[3].push(line[captures.get(5).unwrap()].to_string()); // Continent
                matches[4].push(line[captures.get(6).unwrap()].to_string()); // TLD
                matches[5].push(line[captures.get(7).unwrap()].to_string()); // Currency

                let mut country_languages: Vec<&str> = line[captures.get(8).to_owned().unwrap()].split(',').collect();
                languages.append(
                    &mut country_languages
                        .into_iter()
                        .filter(|&lang| !lang.is_empty())
                        .map(|lang| {
                            // Remove the country part (en-ZM) and keep only the language part
                            if lang.contains("-") {
                                let parts: Vec<&str> = lang.split("-").collect();
                                return parts[0].to_uppercase();
                            }

                            lang.to_uppercase()
                        })
                        .collect(),
                );

                Ok(true)
            }),
        )
        .unwrap();

    for results in &mut matches {
        results.sort();
        results.dedup();
    }
    languages.sort();
    languages.dedup();

    let tlds = &matches[4].iter().map(|tld| tld.replace(".", "").to_uppercase()).collect();
    update_enum_file(&matches[0], ISO_31661_FILE);
    update_enum_file(&matches[1], ISO_31663_FILE);
    update_enum_file(&matches[2], ISO_3166_FIPS_FILE);
    update_enum_file(&matches[3], ISO_3166_CONTINENT_FILE);
    update_enum_file(tlds, ISO_3166_TLD_FILE);
    update_enum_file(&matches[5], ISO_3166_CURRENCY_FILE);

    languages
}

fn update_enum_file(codes: &Vec<String>, file_path: &str) {
    println!("Updating {}", file_path);

    let codes_enum: String = codes.join(",\n    ");
    let file: String = fs::read_to_string(file_path).unwrap();
    let matcher = RegexMatcher::new(r"(?ms).*?// ENUM START\n(.*)\s+//.*").expect("Invalid Regex");
    let mut captures = matcher.new_captures().unwrap();

    let mut new_file: String = String::from("");
    SearcherBuilder::new()
        .multi_line(true)
        .build()
        .search_slice(
            &matcher,
            &file.as_bytes(),
            UTF8(|_, result| {
                matcher.captures(result.as_bytes(), &mut captures);
                new_file = file.replace(
                    &result[captures.get(1).unwrap()].to_string(),
                    format!("    {},\n   ", &codes_enum).as_str(),
                );
                Ok(true)
            }),
        )
        .unwrap();

    fs::write(file_path, new_file);
}
