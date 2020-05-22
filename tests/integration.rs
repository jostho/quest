use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const COUNTRIES_JSON_REMOTE: &str =
    "https://raw.githubusercontent.com/mledoze/countries/master/dist/countries.json";
const COUNTRIES_JSON: &str = "target/countries.json";
const COUNTRIES_CSV: &str = "target/countries.csv";

fn prepare_countries_json() -> Result<(), Box<dyn Error>> {
    // clean up first
    if Path::new(COUNTRIES_CSV).exists() {
        fs::remove_file(COUNTRIES_CSV)?;
    }
    // get from the remote location
    if !Path::new(COUNTRIES_JSON).exists() {
        let body = reqwest::blocking::get(COUNTRIES_JSON_REMOTE)?.text()?;
        let _result = fs::write(COUNTRIES_JSON, body);
    }
    Ok(())
}

fn get_line_count(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines().count())
}

#[test]
fn generate_content_for_countries_json() {
    let _result = prepare_countries_json();
    quest::generate_content(COUNTRIES_JSON, COUNTRIES_CSV);

    assert!(Path::new(COUNTRIES_JSON).exists());
    assert!(Path::new(COUNTRIES_CSV).exists());

    let lc_json = get_line_count(COUNTRIES_JSON).unwrap();
    let lc_csv = get_line_count(COUNTRIES_CSV).unwrap();
    // csv file has extra line - for header
    assert_eq!(lc_json, lc_csv - 1);
}
