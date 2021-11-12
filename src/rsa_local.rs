use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme, pkcs8::FromPublicKey, pkcs8::FromPrivateKey};
use rand::rngs::OsRng;
use sha2;
use std::io;

pub fn encrypt_rsa(text: &str) -> io::Result<()> {
    let mut rng = OsRng;
    let mut no_of_bits = String::new();
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
    io::stdin().read_line(&mut no_of_bits)?;
    let mut bits = 0;
    match no_of_bits.as_str().trim() {
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
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    let b_text = text.as_bytes();
    let mut padding: PaddingScheme;
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
        "1" => padding = PaddingScheme::new_pkcs1v15_encrypt(),
        "2" => padding = PaddingScheme::new_oaep::<sha2::Sha256>(),
        _ => {
            eprintln!("Please provide a valid ID.");
            std::process::exit(1);
        }
    };
    let enc_data = public_key.encrypt(&mut rng, padding, &b_text[..]).expect("Failed to encrypt");
    println!("{:?}", String::from_utf8(enc_data));
    Ok(())
}
