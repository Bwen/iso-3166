const GEONAMES_FILENAME: &str = "countryInfo.txt";

#[cfg(feature = "country-embed-db")]
const GEONAMES_DB: &'static [u8] = include_bytes!("../../geonames-files/pd_sample/countryInfo.txt");

use std::str::FromStr;

use crate::country::*;
#[cfg(feature = "language-info")]
use crate::language;

use crate::geonames::FileReader;

#[derive(Debug)]
pub struct Info {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric3: u8,
    pub fips: Fips,
    pub name: String,
    pub capital: String,
    pub size: f32,
    pub population: usize,
    pub continent_code: ContinentCode,
    pub tld: Tld,
    pub currency_code: CurrencyCode,
    pub currency_name: String,
    pub phone: String,
    pub postal_code_format: String,
    pub postal_code_regex: String,
    #[cfg(feature = "language-info")]
    pub languages: Vec<language::Info>,
    #[cfg(not(feature = "language-info"))]
    pub language_codes: String,
    pub geoname_id: u32,
    pub neighbours_country_codes: Vec<Alpha2>,
}

impl<'p, 'b> Info {
    fn get_reader() -> FileReader<'p, 'b> {

        #[cfg(not(feature = "country-embed-db"))]
            return FileReader::from(GEONAMES_FILENAME);

        #[cfg(feature = "country-embed-db")]
            return FileReader::from(GEONAMES_DB);
    }

    pub fn all() -> InfoIterator<'p, 'b>  {
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^[A-Z]{{2}}\t"));

        let boundaries = reader.line_boundaries();
        InfoIterator {
            line_number: boundaries.0,
            last_line_number: boundaries.1,
            reader,
        }
    }
}

pub struct InfoIterator<'p, 'b> {
    line_number: u64,
    last_line_number: u64,
    reader: FileReader<'p, 'b>,
}

impl<'p, 'b> Iterator for InfoIterator<'p, 'b> {
    type Item = Info;

    fn next(&mut self) -> Option<Self::Item> {
        self.line_number += 1;
        if self.line_number > self.last_line_number {
            return None;
        }

        let line = self.reader.read_line(self.line_number);
        Some(transform_line(&line))
    }
}

impl From<Alpha2> for Info {
    fn from(value: Alpha2) -> Info {
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^{}\t", value));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The country Alpha2 {:?} could not be found in: {}", value, GEONAMES_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<Alpha2> for Info {
    fn into(self) -> Alpha2 {
        self.alpha2
    }
}

impl From<Alpha3> for Info {
    fn from(value: Alpha3) -> Info {
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^(.+?\t|\t){{1}}{}\t", value));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The country FIPS {:?} could not be found in: {}", value, GEONAMES_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<Alpha3> for Info {
    fn into(self) -> Alpha3 {
        self.alpha3
    }
}

impl From<Fips> for Info {
    fn from(value: Fips) -> Info {
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^(.+?\t|\t){{3}}{}\t", value));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The country FIPS {:?} could not be found in: {}", value, GEONAMES_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<Fips> for Info {
    fn into(self) -> Fips {
        self.fips
    }
}

impl From<usize> for Info {
    fn from(value: usize) -> Info {
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^(.+?\t|\t){{15}}{}\t", value));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The country Geoname ID {:?} could not be found in: {}", value, GEONAMES_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<usize> for Info {
    fn into(self) -> usize {
        self.geoname_id as usize
    }
}

impl From<Tld> for Info {
    fn from(value: Tld) -> Info {
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^(.+?\t|\t){{9}}{}\t", value));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The country TLD {:?} could not be found in: {}", value, GEONAMES_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<Tld> for Info {
    fn into(self) -> Tld {
        self.tld
    }
}

fn transform_line(line: &String) -> Info {
    let line_parts: Vec<&str> = line.split("\t").collect();

    let mut neighbours_countries: Vec<Alpha2> = vec![];
    if line_parts.get(17).is_some() {
        neighbours_countries = line_parts[17]
            .split(",")
            .into_iter()
            .filter_map(|alpha2| Alpha2::from_str(alpha2).ok())
            .collect::<Vec<Alpha2>>();
    }

    let mut language_codes = String::from("");
    #[cfg(feature = "language-info")]
    let mut languages: Vec<language::Info> = vec![];

    if line_parts.get(15).is_some() {
        language_codes = line_parts[15].to_string();

        #[cfg(feature = "language-info")]
        language::Info::parse_language_codes(&language_codes, &mut languages);
    }

    Info {
        alpha2: Alpha2::from_str(line_parts[0]).unwrap_or(Alpha2::None),
        alpha3: Alpha3::from_str(line_parts[1]).unwrap_or(Alpha3::None),
        numeric3: line_parts[2].parse().unwrap_or(0),
        fips: Fips::from_str(line_parts[3]).unwrap_or(Fips::None),
        name: line_parts[4].to_string(),
        capital: line_parts[5].to_string(),
        size: line_parts[6].parse().unwrap_or(0.0),
        population: line_parts[7].parse().unwrap_or(0),
        continent_code: ContinentCode::from_str(line_parts[8]).unwrap_or(ContinentCode::None),
        tld: Tld::from_str(line_parts[9].replace(".", "").to_uppercase().as_str()).unwrap_or(Tld::None),
        currency_code: CurrencyCode::from_str(line_parts[10]).unwrap_or(CurrencyCode::None),
        currency_name: line_parts[11].to_string(),
        phone: line_parts[12].to_string(),
        postal_code_format: line_parts[13].to_string(),
        postal_code_regex: line_parts[14].to_string(),
        #[cfg(feature = "language-info")]
        languages,
        #[cfg(not(feature = "language-info"))]
        language_codes,
        geoname_id: line_parts[16].parse().unwrap_or(0),
        neighbours_country_codes: neighbours_countries,
    }
}
