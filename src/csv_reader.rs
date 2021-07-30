use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, Searcher, Sink};
use std::env;
use std::path::{Path, PathBuf};

pub (in crate) struct CsvReader<'b> {
    matcher: RegexMatcher,
    path: Option<PathBuf>,
    bytes: Option<&'b [u8]>
}

impl<'b> From<&str> for CsvReader<'b>  {
    fn from(filename: &str) -> CsvReader<'b>  {
        let env_path = env::var("ISO_DATA_DIR").expect("Undefined environment variable ISO_DATA_DIR");
        let data_file = format!("{}/{}", env_path, filename);
        let path = Path::new(&data_file);
        if !path.exists() {
            panic!("The file name {:?} does not exists in ISO_DATA_DIR: {}", filename, data_file)
        }

        let matcher = RegexMatcher::new_line_matcher(r"(?m)^[^#]").expect("Invalid Regexp");
        CsvReader { path: Some(path.to_owned()), matcher, bytes: None }
    }
}

impl<'b> From<&'b [u8]> for CsvReader<'b>  {
    fn from(bytes: &'b [u8]) -> CsvReader<'b>  {
        let matcher = RegexMatcher::new_line_matcher(r"(?m)^[^#]").expect("Invalid Regexp");
        CsvReader { path: None, matcher, bytes: Some(bytes) }
    }
}

impl<'b> CsvReader<'b> {
    pub fn get_path(&self) -> &Option<PathBuf> {
        &self.path
    }

    pub fn set_regex(&mut self, regex: String) {
        self.matcher = RegexMatcher::new_line_matcher(regex.as_str()).expect("Invalid Regexp");
    }

    fn run_search(&self, utf8_fn: impl Sink) {
        if self.bytes.is_some() {
            let _result = Searcher::new().search_slice(&self.matcher, &self.bytes.unwrap(), utf8_fn);
        } else if self.path.is_some() {
            let _result = Searcher::new().search_path(&self.matcher, &self.get_path().as_ref().unwrap(), utf8_fn);
        } else {
            panic!("CsvReader requires at least a path or bytes")
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
