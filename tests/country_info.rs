use std::str::FromStr;

extern crate iso_3166;
use iso_3166::country::{Info, Numeric, Alpha2, Alpha3};

#[test]
fn info_from_numeric() {
    let numeric = Numeric::from_str("N124").expect("Country numeric N124 should exists!");
    let info = Info::from(numeric);
    assert_eq!(info.name, "Canada");
    assert_eq!(info.flag, "data/images/country_flags/CA.svg");
    assert_eq!(info.alpha2, Alpha2::from_str("CA").unwrap());
    assert_eq!(info.alpha3, Alpha3::from_str("CAN").unwrap());

    let test: Numeric = info.into();
    assert_eq!(test, Numeric::from_str("N124").unwrap());
}

#[test]
fn info_from_alpha2() {
    let alpha2 = Alpha2::from_str("CA").expect("Country Alpha2 CA should exists!");
    let info = Info::from(alpha2);
    assert_eq!(info.name, "Canada");
    assert_eq!(info.flag, "data/images/country_flags/CA.svg");
    assert_eq!(info.alpha2, Alpha2::from_str("CA").unwrap());
    assert_eq!(info.alpha3, Alpha3::from_str("CAN").unwrap());

    let test: Alpha2 = info.into();
    assert_eq!(test, Alpha2::from_str("CA").unwrap());
}

#[test]
fn info_from_alpha3() {
    let alpha3 = Alpha3::from_str("CAN").expect("Country Alpha3 CAN should exists!");
    let info = Info::from(alpha3);
    assert_eq!(info.name, "Canada");
    assert_eq!(info.flag, "data/images/country_flags/CA.svg");
    assert_eq!(info.alpha2, Alpha2::from_str("CA").unwrap());
    assert_eq!(info.alpha3, Alpha3::from_str("CAN").unwrap());

    let test: Alpha3 = info.into();
    assert_eq!(test, Alpha3::from_str("CAN").unwrap());
}

#[test]
fn all_iterator() {
    let mut found = 0;
    let countries = Info::all();
    for country in countries {
        if country.alpha2.as_ref() == "CA" {
            found += 1;
            assert_eq!(country.name, "Canada");
            assert_eq!(country.flag, "data/images/country_flags/CA.svg");
            assert_eq!(country.alpha2, Alpha2::from_str("CA").unwrap());
            assert_eq!(country.alpha3, Alpha3::from_str("CAN").unwrap());
        }
    }

    assert_eq!(found, 1);
}
