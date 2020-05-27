use reqwest::blocking;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const COUNTRIES_JSON_REMOTE: &str =
    "https://raw.githubusercontent.com/mledoze/countries/master/dist/countries.json";

fn prepare_countries_json(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // clean up first
    if Path::new(output_path).exists() {
        fs::remove_file(output_path)?;
    }
    // get from the remote location
    if !Path::new(input_path).exists() {
        println!("Getting countries.json from remote");
        let body = blocking::get(COUNTRIES_JSON_REMOTE)?.text()?;
        let _result = fs::write(input_path, body);
    }
    Ok(())
}

fn get_line_count(path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines().count())
}

#[test]
#[ignore]
fn generate_content_for_remote_countries_json() {
    let countries_json = "target/countries.json";
    let countries_csv = "target/countries.csv";

    let _result = prepare_countries_json(&countries_json, &countries_csv);
    quest::generate_content(&countries_json, &countries_csv);

    assert!(Path::new(&countries_json).exists());
    assert!(Path::new(&countries_csv).exists());

    let lc_json = get_line_count(&countries_json).unwrap();
    let lc_csv = get_line_count(&countries_csv).unwrap();
    // csv file has extra line - for header
    assert_eq!(lc_json, lc_csv - 1);
}
