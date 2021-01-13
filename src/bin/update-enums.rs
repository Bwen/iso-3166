use grep_matcher;
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::{Searcher, SearcherBuilder};

use grep_matcher::{Captures, Matcher};
use std::fs;
use std::fs::File;
use std::io::Error;

const DATA_COUNTRYINFO_FILE: &str = "data/countries.csv";
const ISO_31661_FILE: &str = "src/country/alpha2.rs";
const ISO_31662_FILE: &str = "src/country/alpha3.rs";
const ISO_3166_FIPS_FILE: &str = "src/country/numeric.rs";

fn main() {
    update_country_enums();
}

fn update_country_enums() {
    let regex = "(?m)^([A-Z]{2})\t([A-Z]{3})\t([0-9]{1,3})\t(.*)";
    let matcher = RegexMatcher::new_line_matcher(regex).unwrap();
    let mut captures = matcher.new_captures().unwrap();

    let mut matches: Vec<Vec<String>> = vec![vec![], vec![], vec![], vec![], vec![], vec![]];
    Searcher::new()
        .search_path(
            &matcher,
            DATA_COUNTRYINFO_FILE,
            UTF8(|_, line| {
                matcher.captures(line.as_bytes(), &mut captures);
                matches[0].push(line[captures.get(1).unwrap()].to_string()); // Alpha2
                matches[1].push(line[captures.get(2).unwrap()].to_string()); // Alpha3
                matches[2].push(format!("N{}", line[captures.get(3).unwrap()].to_string())); // Numeric
                // matches[3].push(line[captures.get(4).unwrap()].to_string()); // Flag

                Ok(true)
            }),
        )
        .unwrap();

    for results in &mut matches {
        results.sort();
        results.dedup();
    }

    update_enum_file(&matches[0], ISO_31661_FILE);
    update_enum_file(&matches[1], ISO_31662_FILE);
    update_enum_file(&matches[2], ISO_3166_FIPS_FILE);
}

fn update_enum_file(codes: &Vec<String>, file_path: &str) {
    println!("Updating {}", file_path);

    let codes_enum: String = format!("None,\n    {}", codes.join(",\n    "));
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
