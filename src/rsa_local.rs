use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;
use std::io;
use std::fs;
use std::fs::File;
use std::io::{Read,Write}; 

pub fn encrypt_rsa(text: &str) -> io::Result<()> {
    let mut id_of_bits = String::new();
    println!("Of how many bits do you want the key to be?");
    println!("Enter the corresponding ID");
    print!("+----+----------------+
| ID | NUMBER OF BITS |
+----+----------------+
|    |                |
| 1  | 512            |
|    |                |
| 2  | 1024           |
|    |                |
| 3  | 2048           |
|    |                |
| 4  | 3072           |
|    |                |
| 5  | 4096           |
+----+----------------+\n>");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_of_bits)?;
    println!("\n");
    let mut bits;
    match id_of_bits.as_str().trim() {
        "1" => bits = 512,
        "2" => bits = 1024,
        "3" => bits = 2048,
        "4" => bits = 3072,
        "5" => bits = 4096,
        _ => {
            eprintln!("Please provide a valid ID.");
            std::process::exit(1);
        }
    };
    let rsa = Rsa::generate(bits).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    println!("Private key:\n{}", String::from_utf8(private_key).unwrap());
    println!("Public key:\n{}", String::from_utf8(public_key).unwrap());
    let b_text = text.as_bytes();
    let mut padding: Padding;
    let mut id_padding = String::new();
    println!("What padding scheme?");
    println!("Enter the corresponding ID");
    print!("+----+----------------+
| ID | PADDING SCHEME |
+----+----------------+
|    |                |
| 1  | PKCS1v15       |
|    |                |
| 2  | OAEP           |
+----+----------------+\n>");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_padding)?;
    println!("");
    match id_padding.as_str().trim() {
        "1" => padding = Padding::PKCS1,
        "2" => padding = Padding::PKCS1_OAEP,
        _ => {
            eprintln!("Please provide a valid ID.");
            std::process::exit(1);
        }
    };
    let mut enc_data: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(b_text, &mut enc_data, padding).unwrap();
    println!("Where do you want to save the encrypted data?");
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    fs::write(path.trim(), enc_data)?;
    Ok(())
}

pub fn decrypt_rsa(path: &str) -> io::Result<()> {
    let mut f = File::open(path.trim()).expect("Couldn't open file");
    let mut metadata = fs::metadata(path.trim()).expect("Unable to read metadata");
    let mut text_read_vec = vec![0; metadata.len() as usize];
    f.read(&mut text_read_vec).expect("Buffer overflow");
    let mut text_read = &text_read_vec[..];
    let mut id_of_bits = String::new();
    println!("Of how many bits is your key?");
    println!("Enter the corresponding ID");
    print!("+----+----------------+
| ID | NUMBER OF BITS |
+----+----------------+
|    |                |
| 1  | 512            |
|    |                |
| 2  | 1024           |
|    |                |
| 3  | 2048           |
|    |                |
| 4  | 3072           |
|    |                |
| 5  | 4096           |
+----+----------------+\n>");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_of_bits)?;
    println!("");
    let mut bits;
    match id_of_bits.as_str().trim() {
        "1" => bits = 512,
        "2" => bits = 1024,
        "3" => bits = 2048,
        "4" => bits = 3072,
        "5" => bits = 4096,
        _ => {
            eprintln!("Please provide a valid ID.");
            std::process::exit(1);
        }
    };
    println!("Is your key password-protected?");
    print!("+----+---------+
| ID |  ANSWER |
+----+---------+
|    |         |
| 1  |  Yes    |
|    |         |
| 2  |  No     |
+----+---------+\n>");
    io::stdout().flush().unwrap();
    let mut answer_id = String::new();
    io::stdin().read_line(&mut answer_id)?;
    println!("");
    let rsa;
    match answer_id.as_str().trim() {
        "1" => {
            let mut password = String::new();
            let mut path_private = String::new();
            print!("Enter password: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut password)?;
            print!("Enter the path of your private key: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut path_private)?;
            let private_key = fs::read_to_string(path_private.trim()).expect("Unable to read file");
            rsa = Rsa::private_key_from_pem_passphrase(private_key.as_bytes(), password.trim().as_bytes()).unwrap();
            
        },
        "2" => {
            let mut path_private = String::new();
            print!("Enter the path of your private key: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut path_private)?;
            let private_key = fs::read_to_string(path_private.trim()).expect("Unable to read file");
            rsa = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
        }
        _ => {
            eprintln!("Please provide a valid ID");
            std::process::exit(1);
        }
    }
    let mut data: Vec<u8> = vec![0; rsa.size() as usize];
    let mut padding: Padding;
    let mut id_padding = String::new();
    println!("What padding scheme?");
    println!("Enter the corresponding ID");
    print!("+----+----------------+
| ID | PADDING SCHEME |
+----+----------------+
|    |                |
| 1  | PKCS1v15       |
|    |                |
| 2  | OAEP           |
+----+----------------+\n>");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_padding)?;
    println!("");
    match id_padding.as_str().trim() {
        "1" => padding = Padding::PKCS1,
        "2" => padding = Padding::PKCS1_OAEP,
        _ => {
            eprintln!("Please provide a valid ID.");
            std::process::exit(1);
        }
    };
    let _ = rsa.private_decrypt(text_read, &mut data, padding).unwrap();
    println!("Decrypted: {}", String::from_utf8(data).unwrap());
    Ok(())
}
