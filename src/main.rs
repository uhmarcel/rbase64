use std::io;
use std::fs;
use std::io::Read;
use clap::Parser;

mod base64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false, help = "Decode input (default: false)")]
    decode: bool,

    #[arg(short, long, help = "Input file (stdin if missing)")]
    input: Option<String>,

    #[arg(short, long, help = "Output file (stdout if missing)")]
    output: Option<String>,

    #[arg(hide = true)]
    argument: Option<String>
}

fn main() {
    let args: Args = Args::parse();
    let input = parse_input(args.input.or(args.argument));

    let processed = if args.decode {
        base64::decode(&input)
    } else {
        base64::encode(&input)
    };

    match args.output {
        Some(path) => {
            fs::write(path, processed).expect("Failed to write output");
        },
        None => {
            print!("{}", processed);
        }
    }
}

fn parse_input(file: Option<String>) -> String {
    return match file {
        Some(path) => {
            fs::read_to_string(path).expect("Unable to load file")
        },
        None => {
            let mut buffer: Vec<u8> = Vec::new();
            io::stdin().read_to_end(&mut buffer).expect("Failed to read stdin");
            String::from_utf8(buffer).expect("Failed to parse stdin")
        },
    }
}