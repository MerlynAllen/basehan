

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

fn interactive_shell() {

}


fn main() {
    let args = Args::parse();
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer).expect("Failed to read from stdin.");

    if args.interactive {
        interactive_shell();
        return;
    }

    if args.decode {
        // check is string
        let buffer = String::from_utf8(buffer).expect("Invalid input. Expected UTF-8 string for decoding.");
        let result = basehan::decode(buffer);
        // let result = String::from_utf8(result).expect("Internal bugs occurred when decoding.").to_string();
        io::stdout().write_all(&result).expect("Failed to write to stdout.");
    } else {
        let result = basehan::encode(buffer);
        io::stdout()
            .write_all(result.as_bytes())
            .expect("Failed to write to stdout.");
    }
}
