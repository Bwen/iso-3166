use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, Searcher, Sink};
use std::env;
use std::path::Path;

pub struct FileReader<'p, 'b> {
    matcher: RegexMatcher,
    path: Option<&'p str>,
    bytes: Option<&'b [u8]>
}

impl<'p, 'b> From<&'p str> for FileReader<'p, 'b>  {
    fn from(filename: &'p str) -> FileReader<'p, 'b>  {
        let env_path = env::var("GEONAMES_DIR").expect("Undefined environment variable GEONAMES_DIR");
        let geonames_file = format!("{}/{}", env_path, filename);
        let path = Path::new(&geonames_file);
        if !path.exists() {
            panic!(format!("The file name {:?} does not exists in GEONAMES_DIR: {}", filename, geonames_file))
        }

        let matcher = RegexMatcher::new_line_matcher(r"(?m)^[^#]").expect("Invalid Regexp");
        FileReader { path: Some(filename), matcher, bytes: None }
    }
}

impl<'p, 'b> From<&'b [u8]> for FileReader<'p, 'b>  {
    fn from(bytes: &'b [u8]) -> FileReader<'p, 'b>  {
        let matcher = RegexMatcher::new_line_matcher(r"(?m)^[^#]").expect("Invalid Regexp");
        FileReader { path: None, matcher, bytes: Some(bytes) }
    }
}

impl<'p, 'b> FileReader<'p, 'b> {
    pub fn set_regex(&mut self, regex: String) {
        self.matcher = RegexMatcher::new_line_matcher(regex.as_str()).expect("Invalid Regexp");
    }

    fn run_search(&self, utf8_fn: impl Sink) {
        if self.bytes.is_some() {
            Searcher::new().search_slice(&self.matcher, &self.bytes.unwrap(), utf8_fn);
        } else if self.path.is_some() {
            let env_path = env::var("GEONAMES_DIR").expect("Undefined environment variable GEONAMES_DIR");
            Searcher::new().search_path(&self.matcher, format!("{}/{}", env_path, &self.path.unwrap()), utf8_fn);
        } else {
            panic!("Geonames FileReader requires at least a path or bytes")
        }
    }

    pub fn line_boundaries(&self) -> (u64, u64) {
        let mut first_line_number: u64 = 0;
        let mut last_line_number: u64 = 0;
        let utf8_fn = UTF8(|lnum, _line| {
            if first_line_number == 0 {
                first_line_number = lnum - 1;
            }

            last_line_number = lnum;
            Ok(true)
        });

        self.run_search(utf8_fn);

        (first_line_number, last_line_number)
    }

    pub fn read_line(&self, line_number: u64) -> String {
        let mut line_content = String::from("");
        let utf8_fn = UTF8(|lnum, line| {
            if line_number == lnum {
                line_content = line.to_string();
                Ok(false)
            } else {
                Ok(true)
            }
        });

        self.run_search(utf8_fn);

        line_content
    }

    pub fn read_lines(&self, lines: &mut Vec<String>) {
        let utf8_fn = UTF8(|_lnum, line| {
            lines.push(line.to_string());

            Ok(true)
        });

        self.run_search(utf8_fn);
    }
}
