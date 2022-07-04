use clap::Parser;

/// Read cli args
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Generate csv
    #[clap(short, long)]
    generate: bool,

    /// List csv
    #[clap(short, long)]
    list: bool,

    /// Input file path
    #[clap(short, long, value_parser, validator = quest::is_valid_file)]
    input: String,

    /// Output file path
    #[clap(short, long, value_parser)]
    output: Option<String>,

    /// Number of questions
    #[clap(short, long, default_value_t = 10, value_parser = clap::value_parser!(u8).range(1..100))]
    count: u8,
}

fn main() {
    let args = Args::parse();

    if args.generate {
        let default_output_path = quest::get_output_path(&args.input);
        let output_path = args.output.unwrap_or(default_output_path);

        // generate content for quiz
        quest::generate_content(&args.input, &output_path);
    } else if args.list {
        // list content
        quest::get_content(&args.input, true);
    } else {
        // ask a quiz
        quest::ask_quiz(&args.input, args.count);
    }
}
