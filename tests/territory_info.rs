use std::str::FromStr;

extern crate iso_3166;
use iso_3166::territory::{Info, Code};
use iso_3166::country::Alpha2;

#[test]
fn info_from_code() {
    let code = Code::from_str("CA_QC").expect("Territory CA_QC should exists!");
    let info = Info::from(code);
    assert_eq!(info.country, Alpha2::from_str("CA").unwrap());
    assert_eq!(info.code, Code::from_str("CA_QC").unwrap());
    assert_eq!(info.name, "Quebec");
    assert_eq!(format!("{}", info.code), "CA-QC");

    let test: Code = info.into();
    assert_eq!(test, Code::from_str("CA_QC").unwrap());
}

#[test]
fn all_iterator() {
    let mut found = 0;
    let territories = Info::all();
    for territory in territories {
        println!("{:?}", &territory);

        assert_ne!(territory.code, Code::None, "No Territory code should be Code::None");
        if territory.code.as_ref() == "CA_QC" {
            found += 1;
            assert_eq!(territory.country, Alpha2::from_str("CA").unwrap());
            assert_eq!(territory.code, Code::from_str("CA_QC").unwrap());
            assert_eq!(territory.name, "Quebec");
            assert_eq!(format!("{}", territory.code), "CA-QC");
        }
    }

    assert_eq!(found, 1);
}

#[test]
fn for_country_iterator() {
    let mut found = 0;
    let territories = Info::for_country(Alpha2::CA);
    for territory in territories {
        found += 1;
        if territory.code.as_ref() == "CA_QC" {
            assert_eq!(territory.country, Alpha2::from_str("CA").unwrap());
            assert_eq!(territory.code, Code::from_str("CA_QC").unwrap());
            assert_eq!(territory.name, "Quebec");
            assert_eq!(format!("{}", territory.code), "CA-QC");
        }
    }

    assert_eq!(found, 13);
}