use clap::{App, Arg};

const ARG_COMMAND_GENERATE: &str = "generate";
const ARG_INPUT: &str = "input";
const ARG_OUTPUT: &str = "output";

fn main() {
    let args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .arg(
            Arg::with_name(ARG_COMMAND_GENERATE)
                .short("g")
                .long(ARG_COMMAND_GENERATE)
                .help("Generate a csv")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_INPUT)
                .short("i")
                .long(ARG_INPUT)
                .help("Input file path")
                .takes_value(true)
                .validator(quest::is_valid_file)
                .required(true),
        )
        .arg(
            Arg::with_name(ARG_OUTPUT)
                .short("o")
                .long(ARG_OUTPUT)
                .help("Output file path")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    if args.is_present(ARG_COMMAND_GENERATE) {
        // generate content for quiz
        quest::generate_content(
            args.value_of(ARG_INPUT).unwrap(),
            args.value_of(ARG_OUTPUT).unwrap(),
        );
    } else {
        // ask a quiz
        quest::ask_quiz(args.value_of(ARG_INPUT).unwrap());
    }
}
