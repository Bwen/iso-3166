const DATA_FILENAME: &str = "countries.csv";

#[cfg(feature = "embed-dbs")]
const DATA_DB: &'static [u8] = include_bytes!("countries.csv");

use std::str::FromStr;
use crate::country::*;
use crate::csv_reader::CsvReader;

#[derive(Debug)]
pub struct Info {
    pub name: String,
    pub flag: String,
    pub alpha2: Alpha2,
    pub alpha3: Alpha3,
    pub numeric: Numeric,
}

impl<'b> Info {
    fn get_reader() -> CsvReader<'b> {

        #[cfg(not(feature = "embed-dbs"))]
            return CsvReader::from(DATA_FILENAME);

        #[cfg(feature = "embed-dbs")]
            return CsvReader::from(DATA_DB);
    }

    pub fn all() -> InfoIterator<'b>  {
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

pub struct InfoIterator<'b> {
    line_number: u64,
    last_line_number: u64,
    reader: CsvReader<'b>,
}

impl<'b> Iterator for InfoIterator<'b> {
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
            // Technically this should never fail as the script that populates enums is the csv
            panic!("The country Alpha2 {:?} could not be found in: {:?}", value, reader.get_path());
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
            // Technically this should never fail as the script that populates enums is the csv
            panic!("The country Alpha3 {:?} could not be found in: {:?}", value, reader.get_path());
        }

        transform_line(&lines[0])
    }
}

impl Into<Alpha3> for Info {
    fn into(self) -> Alpha3 {
        self.alpha3
    }
}

impl From<Numeric> for Info {
    fn from(value: Numeric) -> Info {
        let mut reader = Self::get_reader();
        let mut numeric = format!("{}", value);
        numeric.remove(0);
        reader.set_regex(format!("(?m)^(.+?\t|\t){{2}}{}\t", numeric));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            // Technically this should never fail as the script that populates enums is the csv
            panic!("The country Numeric {:?} could not be found in: {}", numeric, DATA_FILENAME);
        }

        transform_line(&lines[0])
    }
}

impl Into<Numeric> for Info {
    fn into(self) -> Numeric {
        self.numeric
    }
}

fn transform_line(line: &String) -> Info {
    let line_parts: Vec<&str> = line.split("\t").collect();
    let numeric = format!("N{}", &line_parts[2]);
    Info {
        name: line_parts[3].trim().to_string(),
        flag: line_parts[4].trim().to_string(),
        alpha2: Alpha2::from_str(line_parts[0]).unwrap_or(Alpha2::None),
        alpha3: Alpha3::from_str(line_parts[1]).unwrap_or(Alpha3::None),
        numeric: Numeric::from_str(numeric.as_str()).unwrap_or(Numeric::None),
    }
}
