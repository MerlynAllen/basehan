pub mod base_han {
    use std::result;

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
        let is_multibyte_tail = basehan[basehan.len() - 1] as u32 == MULTIBYTE_SIGN;
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

        let index = basehan[basehan.len() - 1] as u32 - BASE_OFFSET;
        buff = (buff << tail_bits) | (index & (0xFFFFFFFF >> (32 - tail_bits)));
        // assert!((bit_pointer + tail_bits) % 8 == 0);
        if is_multibyte_tail {
            let byte = (buff >> 8) & 0xFF;
            result.push(byte as u8);
        }
        let byte = buff & 0xFF;
        result.push(byte as u8);
        result
    }

    pub struct BaseHanEncoder {
        buff: u32,
        nbits: u32,
    }
    impl BaseHanEncoder {
        pub fn new() -> Self {
            Self { buff: 0, nbits: 0 }
        }
        pub fn update(mut self, buff: Vec<u8>) -> Vec<char> {
            let mut result = Vec::new();
            for i in 0..buff.len() {
                self.buff = self.buff << 8 | buff[i] as u32;
                self.nbits += 8;
                while self.nbits >= 13 {
                    self.nbits -= 13;
                    let index = (self.buff >> self.nbits) & 0x1FFF;
                    result.push(char::from_u32(index + BASE_OFFSET).unwrap());
                }
            }
            result
        }
        pub fn finish(self) -> Vec<char> {
            let mut result = Vec::new();
            match self.nbits {
                0 => (),
                1..=8 => {
                    let index = self.buff & (0xFFFFFFFF >> (32 - self.nbits));
                    result.push(char::from_u32(index + BASE_OFFSET).unwrap());
                }
                9..=12 => {
                    let index = self.buff & (0xFFFFFFFF >> (32 - self.nbits));
                    result.push(char::from_u32(index + BASE_OFFSET).unwrap());
                    result.push(char::from_u32(MULTIBYTE_SIGN).unwrap());
                }
                _ => unreachable!(),
            }
            result
        }
    }

    pub struct BaseHanDecoder {
        buff: u32,
        nbits: u32,
    }
    impl BaseHanDecoder {
        pub fn new() -> Self {
            Self { buff: 0, nbits: 0 }
        }
        pub fn update(mut self, buff: Vec<char>) -> Vec<u8> {
            let mut result = Vec::new();
            for i in 0..buff.len() {
                let index = buff[i] as u32 - BASE_OFFSET;
                self.buff = (self.buff << 13) | (index & 0x1FFF);
                self.nbits += 13;
                while self.nbits >= 8 {
                    self.nbits -= 8;
                    let byte = (self.buff >> self.nbits) & 0xFF;
                    result.push(byte as u8);
                }
            }
            result
        }
        pub fn finish(mut self, buff: Vec<char>) -> Vec<u8> {
            let mut result = Vec::new();
            // last byte(s)
            let tail_bits = (8 * if buff[buff.len() - 1] as u32 == MULTIBYTE_SIGN {
                2
            } else {
                1
            }) - self.nbits;

            let index = buff[buff.len() - 1] as u32 - BASE_OFFSET;
            self.buff = (self.buff << tail_bits) | (index & (0xFFFFFFFF >> (32 - tail_bits)));
            // assert!((bit_pointer + tail_bits) % 8 == 0);
            if buff[buff.len() - 1] as u32 == MULTIBYTE_SIGN {
                let byte = (self.buff >> 8) & 0xFF;
                result.push(byte as u8);
            }
            let byte = self.buff & 0xFF;
            result.push(byte as u8);
            result
        }
    }
}

use clap::Parser;
use std::io::{self, stdin, Read, Stdin, Write};

use crate::base_han::BASE_OFFSET;
// Base-Han is a command line tool to encode/decode binary data to/from Base-Han.
#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    // Whether encode or docode
    #[clap(short, long, default_value = "false")]
    decode: bool,
}



fn main() {
    let args = Args::parse();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    if args.decode {
        let result = base_han::decode(buffer);
        let result = String::from_utf8(result).unwrap().to_string();
        io::stdout().write_all(result.as_bytes()).unwrap();
    } else {
        let result = base_han::encode(buffer.as_bytes().to_vec());
        io::stdout()
            .write_all(result.iter().collect::<String>().as_bytes())
            .unwrap();
    }
}
