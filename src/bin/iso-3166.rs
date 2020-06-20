extern crate iso_3166;
use iso_3166::country::*;
use std::convert::From;
use strum::IntoEnumIterator;

/// Utility to keep the enums up to date with geonames updates
#[derive(StructOpt, Debug)]
#[structopt(name = "iso-3166")]
struct Opts {}

fn main() {

}
