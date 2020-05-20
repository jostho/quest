use ansi_term::Colour::{Green, Red};
use csv::ReaderBuilder;
use csv::WriterBuilder;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::process;
use std::time::Instant;

const MAX_COUNT: u8 = 100; // stay under 255, u8::MAX
const NUMBER_OF_OPTIONS: u8 = 4;

#[derive(Serialize, Deserialize, Debug)]
struct SourceCountry {
    cca2: String,
    cca3: String,
    ccn3: String,
    name: Name,
    capital: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    common: String,
    official: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    cca2: String,
    cca3: String,
    ccn3: String,
    name_common: String,
    name_official: String,
    capital: String,
}

impl From<SourceCountry> for Country {
    fn from(source: SourceCountry) -> Country {
        let capital = if !source.capital.is_empty() {
            source.capital[0].clone()
        } else {
            String::from("")
        };
        Country {
            cca2: source.cca2,
            cca3: source.cca3,
            ccn3: source.ccn3,
            capital,
            name_common: source.name.common,
            name_official: source.name.official,
        }
    }
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.ccn3 == other.ccn3
    }
}

pub fn is_valid_file(val: String) -> Result<(), String> {
    if Path::new(&val).exists() {
        Ok(())
    } else {
        Err("File does not exist".to_string())
    }
}

pub fn is_valid_count(val: String) -> Result<(), String> {
    let count: u8 = match val.parse() {
        Ok(count) => count,
        Err(e) => return Err(e.to_string()),
    };

    if count < MAX_COUNT {
        Ok(())
    } else {
        Err(format!("Value should be less than {}", MAX_COUNT))
    }
}

pub fn generate_content(input_path: &str, output_path: &str) {
    // read source json
    let result = read_from_json_file(input_path);
    let source_countries = result.unwrap();

    // transform from source
    let countries = transform_from_source(source_countries);

    // write to csv
    let _result = write_to_csv_file(&countries, output_path);
    println!(
        "Generating content from {} into {}",
        input_path, output_path
    );
}

fn read_from_json_file(path: &str) -> Result<Vec<SourceCountry>, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let buf_reader = BufReader::new(file);
    let source_countries = serde_json::from_reader(buf_reader)?;
    Ok(source_countries)
}

fn transform_from_source(source_countries: Vec<SourceCountry>) -> Vec<Country> {
    let mut countries = Vec::new();
    for source_country in source_countries {
        let country = Country::from(source_country);
        countries.push(country);
    }
    countries
}

fn write_to_csv_file(countries: &[Country], path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().delimiter(b'|').from_path(path)?;
    for country in countries {
        writer.serialize(country)?;
    }
    writer.flush()?;
    Ok(())
}

pub fn ask_quiz(input_path: &str, count: u8) {
    let result = read_from_csv_file(input_path);
    let all_countries = result.unwrap();

    let countries = validate_capital(all_countries);

    if countries.len() > count as usize && countries.len() > NUMBER_OF_OPTIONS as usize {
        println!("Asking quiz using {}", input_path);
        let _result = pop_quiz(&countries, count);
    } else {
        eprintln!("Not enough questions in {}", input_path);
        process::exit(2);
    }
}

fn read_from_csv_file(path: &str) -> Result<Vec<Country>, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let buf_reader = BufReader::new(file);
    let mut reader = ReaderBuilder::new().delimiter(b'|').from_reader(buf_reader);
    let mut countries = Vec::new();
    for result in reader.deserialize() {
        let record: Country = result?;
        countries.push(record);
    }
    Ok(countries)
}

fn validate_capital(countries: Vec<Country>) -> Vec<Country> {
    let mut valid_countries = Vec::new();
    for country in countries {
        // check if the capital is empty
        // check if capital matches the name of country
        if !country.capital.is_empty()
            && !country.capital.contains(&country.name_common)
            && !country.name_common.contains(&country.capital)
        {
            valid_countries.push(country);
        }
    }
    valid_countries
}

fn pop_quiz(countries: &[Country], count: u8) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut selections = Vec::new();
    let mut q_count: u8 = 0;
    let mut correct_answer_count: u8 = 0;
    let mut done = false;

    let start_time = Instant::now();
    while !done {
        let q_index = rng.gen_range(0, countries.len());
        let selection = &countries[q_index];
        let mut options: Vec<&Country> = countries
            .choose_multiple(&mut rng, NUMBER_OF_OPTIONS as usize - 1)
            .collect();
        // check if the options already has the selected answer
        // check if question is already asked
        if options.contains(&selection) || selections.contains(&selection) {
            // skip, retry with another question
            continue;
        }
        // it's a GO
        q_count += 1;
        selections.push(selection);
        options.push(selection);
        options.shuffle(&mut rng);
        println!(
            "Question {}/{}: which country's capital is {} ?",
            q_count, count, selection.capital
        );
        println!("Options:");
        for (pos, elem) in options.iter().enumerate() {
            println!("{}. {}", pos + 1, elem.name_common);
        }
        let mut input = String::new();
        let _result = io::stdin().read_line(&mut input);
        let input: u8 = input.trim().parse().unwrap_or(0);
        let mut verdict = Red.paint("wrong");
        if input >= 1
            && input <= NUMBER_OF_OPTIONS
            && selection.name_common == options[input as usize - 1].name_common
        {
            correct_answer_count += 1;
            verdict = Green.paint("correct");
        }
        println!(
            "Your answer #{} is {}. Correct answer is {}",
            input, verdict, selection.name_common
        );
        if q_count == count {
            done = true;
        }
    }
    println!(
        "Final score: {}/{} . Time: {}s",
        correct_answer_count,
        count,
        start_time.elapsed().as_secs()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_file_for_readme() {
        let result = is_valid_file("README.md".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn is_valid_file_for_does_not_exist() {
        let result = is_valid_file("does_not_exist.txt".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File does not exist");
    }

    #[test]
    fn is_valid_count_for_1() {
        let result = is_valid_count("1".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn is_valid_count_for_100() {
        let result = is_valid_count("100".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Value should be less than 100");
    }

    #[test]
    fn is_valid_count_for_foo() {
        let result = is_valid_count("foo".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid digit found in string");
    }

    #[test]
    fn transform_from_source_for_2_countries() {
        let abw_name = Name {
            common: String::from("Aruba"),
            official: String::from("Aruba"),
        };
        let abw = SourceCountry {
            cca2: String::from("AW"),
            cca3: String::from("ABW"),
            ccn3: String::from("533"),
            name: abw_name,
            capital: vec![String::from("Oranjestad")],
        };
        let zaf_name = Name {
            common: String::from("South Africa"),
            official: String::from("Republic of South Africa"),
        };
        let zaf = SourceCountry {
            cca2: String::from("ZA"),
            cca3: String::from("ZAF"),
            ccn3: String::from("710"),
            name: zaf_name,
            capital: vec![
                String::from("Pretoria"),
                String::from("Bloemfontein"),
                String::from("Cape Town"),
            ],
        };

        let mut source_countries = Vec::new();
        source_countries.push(abw);
        source_countries.push(zaf);

        let countries = transform_from_source(source_countries);

        assert_eq!(countries.len(), 2);
        assert_eq!(countries[0].name_common, "Aruba");
        assert_eq!(countries[0].ccn3, "533");
        assert_eq!(countries[1].name_official, "Republic of South Africa");
        assert_eq!(countries[1].capital, "Pretoria");
    }

    #[test]
    fn validate_capital_for_5_countries() {
        let abw = Country {
            cca2: String::from("AW"),
            cca3: String::from("ABW"),
            ccn3: String::from("533"),
            name_common: String::from("Aruba"),
            name_official: String::from("Aruba"),
            capital: String::from("Oranjestad"),
        };
        let ata = Country {
            cca2: String::from("AQ"),
            cca3: String::from("ATA"),
            ccn3: String::from("010"),
            name_common: String::from("Antarctica"),
            name_official: String::from("Antarctica"),
            capital: String::from(""),
        };
        let gnb = Country {
            cca2: String::from("GW"),
            cca3: String::from("GNB"),
            ccn3: String::from("624"),
            name_common: String::from("Guinea-Bissau"),
            name_official: String::from("Republic of Guinea-Bissau"),
            capital: String::from("Bissau"),
        };
        let gtm = Country {
            cca2: String::from("GT"),
            cca3: String::from("GTM"),
            ccn3: String::from("320"),
            name_common: String::from("Guatemala"),
            name_official: String::from("Republic of Guatemala"),
            capital: String::from("Guatemala City"),
        };
        let zwe = Country {
            cca2: String::from("ZW"),
            cca3: String::from("ZWE"),
            ccn3: String::from("716"),
            name_common: String::from("Zimbabwe"),
            name_official: String::from("Republic of Zimbabwe"),
            capital: String::from("Harare"),
        };

        let mut all_countries = Vec::new();
        all_countries.push(abw);
        all_countries.push(ata);
        all_countries.push(gnb);
        all_countries.push(gtm);
        all_countries.push(zwe);

        let countries = validate_capital(all_countries);

        assert_eq!(countries.len(), 2);
        assert_eq!(countries[0].capital, "Oranjestad");
        assert_eq!(countries[1].capital, "Harare");
    }
}
