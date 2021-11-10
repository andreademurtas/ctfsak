mod lib;
use std::env::*;

fn main() {
    if args().count() == 1 { lib::draw_banner() };

    let matches = lib::parse_arguments();
    
    if matches.is_present("file"){

        if let Some(c) = matches.value_of("encode") {
            lib::encode_from_file(c, matches.value_of("INPUT").unwrap()).unwrap();
        }

        if let Some(c) = matches.value_of("decode") {
            lib::decode_from_file(c, matches.value_of("INPUT").unwrap()).unwrap();
        }
    
    }
    else {

        if let Some(c) = matches.value_of("encode") {
            lib::encode(c, matches.value_of("INPUT").unwrap());
        }

        if let Some(c) = matches.value_of("decode") {
            lib::decode(c, matches.value_of("INPUT").unwrap());
        }
    }
}

