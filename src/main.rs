mod utils;
mod rsa_local;

fn main() { 

    let matches = utils::parse_arguments();
    
    if matches.is_present("file"){

        if let Some(c) = matches.value_of("encode") {
            utils::encode_from_file(c, matches.value_of("INPUT").unwrap()).unwrap();
        }

        if let Some(c) = matches.value_of("decode") {
            utils::decode_from_file(c, matches.value_of("INPUT").unwrap()).unwrap();
        }

        if let Some(c) = matches.value_of("encrypt") {
            
        }
        
        if let Some(c) = matches.value_of("decrypt") {

        }
    
    }
    else {

        if let Some(c) = matches.value_of("encode") {
            utils::encode(c, matches.value_of("INPUT").unwrap());
        }

        if let Some(c) = matches.value_of("decode") {
            utils::decode(c, matches.value_of("INPUT").unwrap());
        }
        
        if let Some(c) = matches.value_of("encrypt") {
            utils::encrypt(c, matches.value_of("INPUT").unwrap());
        }

        if let Some(c) = matches.value_of("decrypt") {
            utils::decrypt(c, matches.value_of("INPUT").unwrap());
        }
    }
}

