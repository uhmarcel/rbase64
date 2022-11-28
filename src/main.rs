use clap::Parser;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::process::exit;

const BUFFER_SIZE: usize = 10 * 48 * 1024;

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
    let args = Args::parse();
    let result = process_args(args);

    if let Err(error) = result {
        eprintln!("Error: {}", error);
        exit(1);
    }
}

fn process_args(args: Args) -> Result<(), Box<dyn Error>> {
    let mut reader = get_reader(&args.input.or(args.argument))?;
    let mut writer = get_writer(&args.output)?;

    loop {
        let bytes_read = {
            let buffer = reader.fill_buf()?;

            if args.decode {
                let in_utf8 = std::str::from_utf8(buffer)?;
                writer.write(&rbase64::decode(in_utf8)?)
            } else {
                writer.write(rbase64::encode(buffer).as_bytes())
            }?;
            buffer.len()
        };

        if bytes_read == 0 {
            break;
        }

        reader.consume(bytes_read);
    }
    Ok(())
}

fn get_writer(output: &Option<String>) -> Result<Box<dyn Write>, std::io::Error> {
    let writer: Box<dyn Write> = match output {
        Some(path) => {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?;

            Box::new(BufWriter::with_capacity(BUFFER_SIZE, file))
        }
        None => Box::new(BufWriter::with_capacity(BUFFER_SIZE, stdout().lock())),
    };
    Ok(writer)
}

fn get_reader(file: &Option<String>) -> Result<Box<dyn BufRead>, std::io::Error> {
    let reader: Box<dyn BufRead> = match file {
        Some(path) => {
            let file = File::open(path)?;

            Box::new(BufReader::with_capacity(BUFFER_SIZE, file))
        }
        None => Box::new(BufReader::with_capacity(BUFFER_SIZE, stdin().lock())),
    };
    Ok(reader)
}
