use std::io::{self, Write};

enum InputType {
    Str, Int
}

fn main() {
    // Get command line args
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[!] No input input type specified!");
        // Might move the exit() into usage()
        usage(&args[0]); std::process::exit(1);
    }

    let        key = args[2].clone();
    let mut  input = String::new();
    let     intype = match &*args[1] {
        "str" | "string"  => InputType::Str,
        "int" | "integer" => InputType::Int,
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
    print!("[<]  Input: "); io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let output = dna_encode(&input, intype, key);

    println!("[>] Output: {}", output);
}

fn dna_encode(input: &str, intype: InputType, key: Vec<char>) -> String {
    match intype {
        InputType::Str => {
            dna_encode_string(input, key)
        }
        InputType::Int => {
            dna_encode_integer(input.trim().parse::<u32>().unwrap(), key)
        }
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

    // Bit ugly, but it just reverses the string and returns it
    reverse_string(&out)
}

fn usage(path: &str) {
    println!("[i] Usage: {} <input format: string/int> <key>", path);
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