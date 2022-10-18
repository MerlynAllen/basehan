pub mod base_han {
    pub const BASE_OFFSET: u32 = 0x4e00;
    pub const MULTIBYTE_SIGN: u32 = 0x8e00;
    pub fn encode(raw: Vec<u8>) -> Vec<char> {
        let mut result = Vec::new();
        let mut buff = 0u32;
        let mut bit_pointer = 0;

        for i in 0..raw.len() {
            buff = buff << 8 | raw[i] as u32;
            bit_pointer += 8;
            while bit_pointer >= 13 {
                bit_pointer -= 13;
                let index = (buff >> bit_pointer) & 0x1FFF;
                result.push(char::from_u32(index + BASE_OFFSET).unwrap());
            }
        }
        match bit_pointer {
            0 => (),
            1..=8 => {
                let index = buff & (0xFFFFFFFF >> (32 - bit_pointer));
                result.push(char::from_u32(index + BASE_OFFSET).unwrap());
            }
            9..=12 => {
                let index = buff & (0xFFFFFFFF >> (32 - bit_pointer));
                result.push(char::from_u32(index + BASE_OFFSET).unwrap());
                result.push(char::from_u32(MULTIBYTE_SIGN).unwrap());
            }
            _ => unreachable!(),
        }
        result
    }
    pub fn decode(basehan: String) -> Vec<u8> {
        // check multi bytes tail
        let mut basehan: Vec<char> = basehan.chars().collect();
        let is_multibyte_tail =
            basehan[basehan.len() - 1] == char::from_u32(MULTIBYTE_SIGN).unwrap();
        if is_multibyte_tail {
            basehan.pop();
        }
        // let total_bytes = basehan.len() * 13 / 8 - if is_multibyte_tail { 0 } else { 1 };
        // let tail_bits = total_bytes * 8 % 13;
        let mut result = Vec::new();
        let mut buff = 0u32;
        let mut bit_pointer = 0;

        for i in 0..basehan.len() - 1 {
            let index = basehan[i] as u32 - BASE_OFFSET;
            buff = (buff << 13) | (index & 0x1FFF);
            bit_pointer += 13;
            while bit_pointer >= 8 {
                bit_pointer -= 8;
                let byte = (buff >> bit_pointer) & 0xFF;
                result.push(byte as u8);
            }
        }
        // last byte(s)
        let tail_bits = (8 * if is_multibyte_tail { 2 } else { 1 }) - bit_pointer;

        let index = (basehan[basehan.len() - 1] as i32 - BASE_OFFSET as i32) as u32;
        buff = (buff << tail_bits) | (index & (0xFFFFFFFF >> (32 - tail_bits)));
        // assert!((bit_pointer + tail_bits) % 8 == 0);
        if is_multibyte_tail {
            let byte = (buff >> 8) & 0xFF;
            result.push(byte as u8);
            buff = buff >> 8;
        }
        let byte = buff & 0xFF;
        result.push(byte as u8);
        result
    }
}

use clap::{Parser};
use std::io::{self, stdin, Stdin, Read, Write};
// Base-Han is a command line tool to encode/decode binary data to/from Base-Han.
#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    // Whether encode or docode
    #[clap(short, long, default_value="false")]
    decode: bool,
}

fn main() {
    let args = Args::parse();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    if args.decode {
        let result = base_han::decode(buffer);
        let result = String::from_utf8_lossy(result.as_slice()).to_string();
        io::stdout().write_all(result.as_bytes()).unwrap();
    } else {
        let result = base_han::encode(buffer.as_bytes().to_vec());
        io::stdout().write_all(result.iter().collect::<String>().as_bytes()).unwrap();
    }
}
