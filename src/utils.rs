use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};
use base64;
use urlencoding; 
use std::fs::File;
use std::io::prelude::*;
use crate::rsa_local;

static BANNER: &str = "
          __     _____                           __    
  ____  _/  |_ _/ ____\\           ___________   |  | __
_/ ___\\ \\   __\\\\   __\\   ______  /  ___/\\__  \\  |  |/ /
\\  \\___  |  |   |  |    /_____/  \\___ \\  / __ \\_|    < 
 \\___  > |__|   |__|            /____  >(____  /|__|_ \\
     \\/                              \\/      \\/      \\/
";

pub fn parse_arguments() -> ArgMatches {

    let matches = App::new("🚩 CTF Swiss Army Knife")
        .max_term_width(0)
        .before_help(BANNER)
        .version("1.0")
        .author("Andrea De Murtas <ypno25@protonmail.com>")
        .about("A tool to speed up common operations needed during CTFs")
        .arg(Arg::new("encode")
            .long("encode")
            .takes_value(true)
            .value_name("ENCODING")
            .about("specifies we want to encode in ENCODING encoding "))
        .arg(Arg::new("decode")
            .long("decode")
            .takes_value(true)
            .value_name("ENCODING")
            .about("specifies we want to decode from ENCODING encoding"))
        .arg(Arg::new("encrypt")
            .long("encrypt")
            .takes_value(true)
            .value_name("ALGORITHM")
            .about("specifies we want to encrypt using ALGORITHM algorithm (not implemented yet)"))
        .arg(Arg::new("decrypt")
            .long("decrypt")
            .takes_value(true)
            .value_name("ALGORITHM")
            .about("specifies we want to decrypt using ALGORITHM algorithm (not implemented yet)"))
        .arg(Arg::new("INPUT")
            .about("sets the input string to use, or specifies the file from where to read the input (if flag '-f' is specified)")
            .required(true)
            .index(1))
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .about("If set, INPUT must specify an input file rather than the input itself"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    matches
}

pub fn encode(encoding: &str, text: &str) -> () {
    
    match encoding.to_lowercase().as_str() {
        "base64" => println!("{}", base64::encode(text)),
        "url" => println!("{}", urlencoding::encode(text)),
        _ => println!("Please specify a valid encoding")
    } 

}

pub fn encode_from_file(encoding: &str, filename: &str) -> std::io::Result<()> {
    
    let mut file = File::open(filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;   

    match encoding.to_lowercase().as_str() {
        "base64" => println!("{}", base64::encode(text)),
        "url" => println!("{}", urlencoding::encode(text.as_str())),
        _ => eprintln!("Please specify a valid encoding")
    }

    Ok(())
}

pub fn decode(encoding: &str, text: &str) -> () {
    
    match encoding.to_lowercase().as_str() {
        "base64" => {
            let the_bytes: Vec<u8> = base64::decode(text).unwrap_or_else(|_e| {
                eprintln!("Input is not valid base64");
                std::process::exit(1);
            });
            let s: String = String::from_utf8(the_bytes).unwrap();
            println!("{}", s)
        },
        "url" => println!("{}", urlencoding::decode(text).unwrap_or_else(|_e| {
            eprintln!("Input must be UTF-8");
            std::process::exit(1);
        })),
        _ => eprintln!("Please specify a valid encoding")
    }
}

pub fn decode_from_file(encoding: &str, filename: &str) -> std::io::Result<()> {
   
    let mut file = File::open(filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    match encoding.to_lowercase().as_str() {
        "base64" => {
            let the_bytes: Vec<u8> = base64::decode(text).unwrap_or_else(|_e| {
                eprintln!("Content of file is not valid base64");
                std::process::exit(1);
            });
            let s: String = String::from_utf8(the_bytes).unwrap();
            println!("{}", s)
        },
        "url" => println!("{}", urlencoding::decode(text.as_str()).unwrap_or_else(|_e| {
            eprintln!("File content must be UTF-8");
            std::process::exit(1);
        })),
        _ => eprintln!("Please specify a valid encoding")
    }

    Ok(())
}

pub fn encrypt(algorithm: &str, text: &str) -> () { 
   
    match algorithm.to_lowercase().as_str() {
        "rsa" => {
           rsa_local::encrypt_rsa(text).unwrap(); 
        },
        _ => {
            eprintln!("Please specify a valid algorithm");
        }
    }

}
