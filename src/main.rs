use clap::Parser;
use std::fs;
use std::io;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Decode input (default: false)"
    )]
    decode: bool,

    #[arg(short, long, help = "Input file (stdin if missing)")]
    input: Option<String>,

    #[arg(short, long, help = "Output file (stdout if missing)")]
    output: Option<String>,

    #[arg(hide = true)]
    argument: Option<String>,
}

fn main() {
    let args: Args = Args::parse();
    let input = parse_input(args.input.or(args.argument));

    if args.decode {
        let in_utf8 = String::from_utf8(input).expect("Invalid UTF8");
        let out_bytes = rbase64::decode(&in_utf8);

        output_bytes(args.output, &out_bytes);
    } else {
        let out_utf8 = rbase64::encode(&input);

        output_str(args.output, &out_utf8);
    }
}

#[inline(always)]
fn output_bytes(output: Option<String>, bytes: &[u8]) {
    match output {
        Some(path) => fs::write(path, bytes).expect("Failed to write output"),
        None => {
            print!("{}", String::from_utf8_lossy(bytes));
        }
    }
}

#[inline(always)]
fn output_str(output: Option<String>, value: &str) {
    match output {
        Some(path) => fs::write(path, value).expect("Failed to write output"),
        None => {
            print!("{}", value);
        }
    }
}

fn parse_input(file: Option<String>) -> Vec<u8> {
    match file {
        Some(path) => fs::read(path).expect("Unable to load file"),
        None => {
            let mut buffer: Vec<u8> = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .expect("Failed to read stdin");
            buffer
        }
    }
}
