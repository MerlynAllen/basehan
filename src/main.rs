use clap::Parser;
use std::io::{self, stdin, Read, Stdin, Write};

// Base-Han is a command line tool to encode/decode binary data to/from Base-Han.
#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    // Whether encode or docode
    #[clap(short, long, default_value = "false")]
    decode: bool,
    #[clap(short, long, default_value = "false")]
    interactive: bool,
}

const ENCODE_PROMPT: &str = "encode> ";
const DECODE_PROMPT: &str = "decode> ";

fn interactive_shell(decode: bool) {
    println!("Interactive mode.");
    let stdin = stdin();
    loop {
        print!("{}", if decode { DECODE_PROMPT } else { ENCODE_PROMPT });
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let read_size = stdin.read_line(&mut buffer).unwrap();
        if read_size == 0 {
            break;
        }
        let buffer = buffer.trim();
        if buffer == "exit" {
            break;
        }
        if decode {
            let result = basehan::decode(&buffer.to_string());
            match result {
                Ok(bytes) => {
                    io::stdout().write_all(&bytes).unwrap();
                    println!();
                }
                Err(e) => {
                    println!("Error: Internal error.{:?}", e);
                }
            }
        } else {
            let result = basehan::encode(buffer);
            match result {
                Ok(result) => println!("{}", result),
                Err(err) => println!("Error: Please input a valid BaseHan cipher.{:?}", err),
            }
        }
    }
    println!("Exit");
}

fn main() {
    let args = Args::parse();
    if args.interactive {
        interactive_shell(args.decode);
        return;
    }

    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .expect("Failed to read from stdin.");
    if args.decode {
        // check is string
        let buffer =
            String::from_utf8(buffer).expect("Invalid input. Expected UTF-8 string for decoding.");
        let result = basehan::decode(&buffer)
            .map_err(|e| format!("Failed to decode: {:?}", e))
            .unwrap();
        // let result = String::from_utf8(result).expect("Internal bugs occurred when decoding.").to_string();
        io::stdout()
            .write_all(&result)
            .expect("Failed to write to stdout.");
    } else {
        let result = basehan::encode(buffer).expect("Internal bugs occurred when encoding.");
        io::stdout()
            .write_all(result.as_bytes())
            .expect("Failed to write to stdout.");
    }
}
