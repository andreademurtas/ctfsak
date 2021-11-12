use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;
use std::io;
use std::io::Write;
use std::fs;

pub fn encrypt_rsa(text: &str) -> io::Result<()> {
    let mut id_of_bits = String::new();
    println!("Of how many bits do you want the key to be?");
    println!("Enter the corresponding ID");
    println!("+----+----------------+
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
+----+----------------+");
    io::stdin().read_line(&mut id_of_bits)?;
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
    println!("+----+----------------+
| ID | PADDING SCHEME |
+----+----------------+
|    |                |
| 1  | PKCS1v15       |
|    |                |
| 2  | OAEP           |
+----+----------------+");
    io::stdin().read_line(&mut id_padding)?;
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
