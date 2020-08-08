use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// tests expects "countries" sources at "target/countries"
const COUNTRIES_JSON: &str = "target/countries/dist/countries.json";
const COUNTRIES_COUNT: usize = 229;

fn get_line_count(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines().count())
}

#[test]
#[ignore]
fn generate_content_for_countries_json() {
    let countries_csv = "target/countries-1.csv";
    quest::generate_content(&COUNTRIES_JSON, &countries_csv);
    assert!(Path::new(&countries_csv).is_file());

    let lc_json = get_line_count(&COUNTRIES_JSON).unwrap();
    let lc_csv = get_line_count(&countries_csv).unwrap();
    // csv file has extra line - for header
    assert_eq!(lc_json, lc_csv - 1);
}

#[test]
#[ignore]
fn get_content_for_countries_json() {
    let countries_csv = "target/countries-2.csv";
    quest::generate_content(&COUNTRIES_JSON, &countries_csv);
    assert!(Path::new(&countries_csv).is_file());

    let result = quest::get_content(countries_csv, false);
    assert_eq!(result.len(), COUNTRIES_COUNT);
}
