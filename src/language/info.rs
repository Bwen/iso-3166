const GEONAMES_FILENAME: &str = "iso-languagecodes.txt";

#[cfg(feature = "language-embed-db")]
const GEONAMES_DB: &'static [u8] = include_bytes!("../../geonames-files/pd_sample/iso-languagecodes.txt");

use std::str::FromStr;

use crate::country;
use crate::language::*;
use crate::geonames::FileReader;

#[derive(Debug)]
pub struct Info {
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub alpha3b: Alpha3b,
    pub name: String,
    pub country: country::Alpha2,
}

impl<'p, 'b> Info {
    fn get_reader() -> FileReader<'p, 'b> {
        #[cfg(not(feature = "language-embed-db "))]
            return FileReader::from(GEONAMES_FILENAME);

        #[cfg(feature = "language-embed-db ")]
            return FileReader::from(GEONAMES_DB);
    }

    pub fn parse_language_codes(language_codes: &String, languages: &mut Vec<Info>) {
        let country_languages: Vec<&str> = language_codes.split(",").collect();
        for value in country_languages {
            let lang_code: String;
            let mut alpha2_country = country::Alpha2::None;

            if value.is_empty() {
                continue;
            }

            if value.contains("-") {
                let split: Vec<&str> = value.split("-").collect();
                lang_code = split[0].to_uppercase();
                alpha2_country = country::Alpha2::from_str(split[1]).unwrap_or(country::Alpha2::None);
            } else {
                lang_code = value.to_uppercase();
            }

            let lang_alpha2 = Alpha2::from_str(&lang_code).unwrap_or(Alpha2::None);
            if !lang_alpha2.is_none() {
                let mut lang_info = Info::from(lang_alpha2);
                lang_info.country = alpha2_country;
                languages.push(lang_info);
            } else {
                let lang_alpha3 = Alpha3::from_str(&lang_code).unwrap_or(Alpha3::None);
                if !lang_alpha3.is_none() {
                    let mut lang_info = Info::from(lang_alpha3);
                    lang_info.country = alpha2_country;
                    languages.push(lang_info);
                } else {
                    let lang_alpha3b = Alpha3b::from_str(&lang_code).unwrap_or(Alpha3b::None);
                    if !lang_alpha3.is_none() {
                        let mut lang_info = Info::from(lang_alpha3b);
                        lang_info.country = alpha2_country;
                        languages.push(lang_info);
                    }
                }
            }
        }
    }
}

impl From<Alpha2> for Info {
    fn from(value: Alpha2) -> Info {
        let mut reader = Self::get_reader();
        let code = value.to_string().to_lowercase();
        reader.set_regex(format!("(?m)^(.+?\t|\t){{2}}{}\t", code));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The language Alpha2 {:?} could not be found in: {}", code, GEONAMES_FILENAME);
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
        let code = value.to_string().to_lowercase();
        reader.set_regex(format!("(?m)^{}\t", code));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The language ISO 639-3 {:?} could not be found in: {}", code, GEONAMES_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<Alpha3> for Info {
    fn into(self) -> Alpha3 {
        self.alpha3
    }
}

impl From<Alpha3b> for Info {
    fn from(value: Alpha3b) -> Info {
        let mut reader = Self::get_reader();
        let code = value.to_string().to_lowercase();
        reader.set_regex(format!("(?m)^(.+?\t|\t){{1}}{}\t", code));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            panic!("The language ISO 639-2 {:?} could not be found in: {}", code, GEONAMES_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<Alpha3b> for Info {
    fn into(self) -> Alpha3b {
        self.alpha3b
    }
}

fn transform_line(line: &String) -> Info {
    let line_parts: Vec<&str> = line.split("\t").collect();

    Info {
        alpha2: Alpha2::from_str(&line_parts[2].to_uppercase()).unwrap_or(Alpha2::None),
        alpha3: Alpha3::from_str(&line_parts[0].to_uppercase()).unwrap_or(Alpha3::None),
        alpha3b: Alpha3b::from_str(&line_parts[1].to_uppercase()).unwrap_or(Alpha3b::None),
        name: line_parts[3].trim().to_string(),
        country: country::Alpha2::None,
    }
}
