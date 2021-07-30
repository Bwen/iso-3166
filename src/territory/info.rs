const DATA_FILENAME: &str = "territories.csv";

#[cfg(feature = "embed-dbs")]
const DATA_DB: &'static [u8] = include_bytes!("territories.csv");

use std::str::FromStr;
use crate::territory::*;
use crate::country::Alpha2;
use crate::csv_reader::CsvReader;

#[derive(Debug)]
pub struct Info {
    pub country: Alpha2,
    pub code: Code,
    pub name: String,
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

    pub fn for_country(alpha2: Alpha2) -> InfoIterator<'b>  {
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^{}\t", alpha2));

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

impl From<Code> for Info {
    fn from(value: Code) -> Info {
        let code = format!("{:?}", value);
        let code_parts: Vec<&str> = code.split("_").collect();
        let mut reader = Self::get_reader();
        reader.set_regex(format!("(?m)^(.+?\t|\t){{1}}{}\t", code_parts[1]));

        let mut lines: Vec<String> = vec![];
        reader.read_lines(&mut lines);

        if lines.is_empty() {
            // Technically this should never fail as the script that populates enums is the csv
            panic!("The territory Code {:?} could not be found in: {:?}", value, reader.get_path());
        }

        transform_line(&lines[0])
    }
}

impl Into<Code> for Info {
    fn into(self) -> Code {
        self.code
    }
}

fn transform_line(line: &String) -> Info {
    let line_parts: Vec<&str> = line.split("\t").collect();
    let code = format!("{}_{}", line_parts[0], line_parts[1]);
    Info {
        country: Alpha2::from_str(line_parts[0]).unwrap_or(Alpha2::None),
        code: Code::from_str(code.as_str()).unwrap_or(Code::None),
        name: line_parts[2].trim().to_string(),
    }
}
