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

const MAX_COUNT: u8 = 100; // stay under 255, u8::MAX
const NUMBER_OF_OPTIONS: u8 = 4;

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

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.numeric == other.numeric
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
    // read a json
    let result = read_from_json_file(input_path);
    let iso_3166_1 = result.unwrap();

    // write to csv
    let _result = write_to_csv_file(&iso_3166_1.countries, output_path);
    println!(
        "Generating content from {} into {}",
        input_path, output_path
    );
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

pub fn ask_quiz(input_path: &str, count: u8) {
    let result = read_from_csv_file(input_path);
    let countries = result.unwrap();

    if countries.len() > count as usize && countries.len() > NUMBER_OF_OPTIONS as usize {
        println!("Asking quiz using {}", input_path);
        let _result = pop_quiz(&countries, count);
    } else {
        eprintln!("Input file has fewer rows than number of questions/options");
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

fn pop_quiz(countries: &[Country], count: u8) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut q_count: u8 = 0;
    let mut correct_answer_count: u8 = 0;
    let mut done = false;

    while !done {
        let q_index = rng.gen_range(0, countries.len());
        let selection = &countries[q_index];
        let mut options: Vec<&Country> = countries
            .choose_multiple(&mut rng, NUMBER_OF_OPTIONS as usize - 1)
            .collect();
        // check if the options already has the selected answer
        if options.contains(&selection) {
            // skip, retry with another question
            continue;
        }
        options.push(selection);
        options.shuffle(&mut rng);
        q_count += 1;
        println!(
            "Question {}/{}: which country's code is {} ?",
            q_count, count, selection.alpha_2
        );
        println!("Options:");
        for (pos, elem) in options.iter().enumerate() {
            println!("{}. {}", pos + 1, elem.name);
        }
        let mut input = String::new();
        let _result = io::stdin().read_line(&mut input);
        let input: u8 = input.trim().parse().unwrap_or(0);
        if input >= 1
            && input <= NUMBER_OF_OPTIONS
            && selection.name == options[input as usize - 1].name
        {
            println!(
                "Your answer #{} is correct. Correct answer is {}",
                input, selection.name
            );
            correct_answer_count += 1;
        } else {
            println!(
                "Your answer #{} is wrong. Correct answer is {}",
                input, selection.name
            );
        }
        if q_count == count {
            done = true;
        }
    }
    println!("Final score: {}/{}", correct_answer_count, count);
    Ok(())
}
