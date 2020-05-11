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

#[derive(Serialize, Deserialize, Debug)]
struct Iso31661 {
    #[serde(rename = "3166-1")]
    countries: Vec<Country>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    alpha_2: String,
    alpha_3: String,
    name: String,
    numeric: String,
    #[serde(default)]
    official_name: String,
}

pub fn is_valid_file(val: String) -> Result<(), String> {
    if Path::new(&val).exists() {
        Ok(())
    } else {
        Err("File does not exist".to_string())
    }
}

pub fn generate_content(input_path: &str, output_path: &str) {
    println!(
        "Generating content from {} into {}",
        input_path, output_path
    );
    // read a json
    let result = read_from_json_file(input_path);
    let iso_3166_1 = result.unwrap();

    // write to csv
    let _result = write_to_csv_file(&iso_3166_1.countries, output_path);
}

fn read_from_json_file(path: &str) -> Result<Iso31661, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let buf_reader = BufReader::new(file);
    let iso_3166_1 = serde_json::from_reader(buf_reader)?;
    Ok(iso_3166_1)
}

fn write_to_csv_file(countries: &[Country], path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().delimiter(b'|').from_path(path)?;
    for country in countries {
        writer.serialize(country)?;
    }
    writer.flush()?;
    Ok(())
}

pub fn ask_quiz(input_path: &str) {
    println!("Asking quiz using {}", input_path);
    let result = read_from_csv_file(input_path);
    let countries = result.unwrap();

    let _result = pop_quiz(&countries);
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

fn pop_quiz(countries: &[Country]) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let q_index = rng.gen_range(0, countries.len());
    println!("Got question index: {}", q_index);
    let selection = &countries[q_index];
    //println!("Got country: {:#?}", selection);
    println!("Question: which country's code is {} ?", selection.alpha_2);
    println!("Options:");
    let mut options: Vec<&Country> = countries.choose_multiple(&mut rng, 3).collect();
    options.push(selection);
    options.shuffle(&mut rng);
    //println!("Got options: {:#?}", options);
    for (pos, elem) in options.iter().enumerate() {
        println!("{}. {}", pos + 1, elem.name);
    }
    let mut input = String::new();
    let _result = io::stdin().read_line(&mut input);
    let input: usize = input.trim().parse().unwrap_or(0);
    //println!("Got answer: {:#?}", input);
    if input >= 1 && input <= 4 && selection.name == options[input - 1].name {
        println!(
            "Your answer #{} is correct. Correct answer is {}",
            input, selection.name
        );
    } else {
        println!(
            "Your answer #{} is wrong. Correct answer is {}",
            input, selection.name
        );
    }
    Ok(())
}
