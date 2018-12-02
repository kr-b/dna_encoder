// MIT License
//
// Copyright (c) 2018 kr-b
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::io::{self, Write};

enum PlainTextType {
    Str, Int
}

enum DnaAction {
    Encode, Decode
}

fn main() {
    // Get command line args
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[!] No input input type specified!");
        // Might move the exit() into usage()
        usage(&args[0]); std::process::exit(1);
    }

    let        key = args[3].clone();
    let mut  input = String::new();
    let     action = match &*args[1] {
        "e" | "encode" => DnaAction::Encode,
        "d" | "decode" => DnaAction::Decode,
        _ => {
            println!("[!] Invalid option");
            usage(&args[0]); std::process::exit(1);
        }
    };
    let     intype = match &*args[2] {
        "str" | "string"  => PlainTextType::Str,
        "int" | "integer" => PlainTextType::Int,
        _ => {
            println!("[!] Invalid input type specified");
            usage(&args[0]); std::process::exit(1);
        },
    };

    // Validate key
    if !(is_valid_key(&key)) {
        println!("[!] Invalid key!");
        usage(&args[0]); std::process::exit(1);
    }
    let key: Vec<char> = key.chars().collect();

    // Take input string/integer
    if args.len() >= 5 {
        match intype {
            PlainTextType::Str => {
                for i in 4..args.len() {
                    input.push_str(&args[i]);
                }
                input = input.trim().to_string();
            },
            PlainTextType::Int => {
                input = args[4].clone();
            },
        }
    } else {
        print!("[<]  Input: "); io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
    }
    let output = match action {
        DnaAction::Encode => dna_encode(&input, intype, key),
        DnaAction::Decode => dna_decode(&input, intype, key),
        // _ => { panic! ("[!] Unexpected error matching action")}
    };

    println!("[>] Output: {}", output);
}


fn dna_encode(input: &str, intype: PlainTextType, key: Vec<char>) -> String {
    match intype {
        PlainTextType::Str => dna_encode_string(input, key),
        PlainTextType::Int => dna_encode_integer(input.trim().parse::<u32>().unwrap(), key),
         // _ => { panic! ("[!] Unexpected error matching intype")}
    }
}

fn dna_encode_string(input: &str, key: Vec<char>) -> String {
    let mut out = String::new();

    let v: Vec<u8> = input.trim_right().as_bytes().to_vec();
    let mut nibble = String::new();
    for bit in v {
        let mut n = bit.clone();
        while n != 0 {
            nibble.push(key[(n % 4) as usize]);
            n /= 4;
        }
        nibble = reverse_string(&nibble);
        out += &nibble;
        out.push(' ');
        nibble.clear();
    }

    out
}

fn dna_encode_integer(input: u32, key: Vec<char>) -> String {
    let mut out = String::new();

    let mut n = input.clone();
    while n != 0 {
        out.push(key[(n % 4) as usize]);
        n /= 4;
    }

    reverse_string(&out)
}

fn dna_decode(input: &str, intype: PlainTextType, key: Vec<char>) -> String {
    let mut out = String::new();

    let mut v: Vec<u8> = input.trim_right().as_bytes().to_vec();

    // Unask input
    for bit in &mut v {
        if *bit != ' ' as u8 {
            *bit = match *bit as char {
                c @ 'A' |
                c @ 'C' |
                c @ 'G' |
                c @ 'T'  => key.iter().position(|&x| x == c).unwrap() as u8,
                _ => {
                    panic!("[!] Invalid DNA sequence");
                },
            }
        };
    }
    v.reverse();

    match intype {
        PlainTextType::Str => dna_decode_string(v, key),
        PlainTextType::Int => dna_decode_integer(v, key).to_string(),
         // _ => { panic! ("[!] Unexpected error matching intype")}
    }
}

fn dna_decode_integer(input: Vec<u8>, key: Vec<char>) -> u32 {
    let mut out = 0 as u32;
    let base: u32 = 4;
    for (i, bit) in input.iter().enumerate() {
        out += (base.pow(i as u32)) * (*bit as u32);
    }

    out
}

fn dna_decode_string(input: Vec<u8>, key: Vec<char>) -> String {
    let mut out = String::new();
    let mut nibble = vec![];
    let mut letter: char;
    let mut count = 0;

    for (i, bit) in input.iter().enumerate() {
        if count == 4 || i+1 == input.len() {
            if (i+1) == input.len() { nibble.push(bit.clone()); }
            count = 0;
            for x in &nibble {
                println!("{}",x);
            }
            letter = u32_to_u8_array(dna_decode_integer(nibble.clone(), key.clone()))[3] as char;
            println!("{}", letter);
            out.push(letter);
            nibble.clear();
            continue;
        } else {
            nibble.push(bit.clone());
        }
        count += 1;
    }

    reverse_string(&out)
}

fn usage(path: &str) {
    println!("[i] Usage: {} <encode/decode> <type: string/int> <key> [input]", path);
    println!("[i] Example to convert a string:");
    println!("[-] {} str CGAT", path);
    println!("[i] Example to convert an integer:");
    println!("[-] {} integer ATGC", path);
}

fn is_valid_key(key: &str) -> bool {
    key.len() == 4    &&
    key.contains('A') &&
    key.contains('C') &&
    key.contains('G') &&
    key.contains('T')
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn u32_to_u8_array(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}
