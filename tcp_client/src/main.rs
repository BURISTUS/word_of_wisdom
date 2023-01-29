use rand::{rngs::OsRng, Rng};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Read, Write},
    net::TcpStream,
};

fn main() -> Result<(), io::Error> {
    let mut stream = TcpStream::connect("0.0.0.0:8000")?;
    let mut buffer = String::new();
    let mut nonce = [0u8; 8];
    let mut rng = OsRng;
    rng.fill(&mut nonce);

    let mut nonce = u64::from_le_bytes(nonce);
    let digest = Sha256::new();
    let mut proof_of_work = String::new();
    let zero_string = "0".repeat(4);

    while !proof_of_work.starts_with(&zero_string) {
        let mut digest = digest.clone(); // reset the digest before updating it
        digest.update(format!("{}{}", buffer, nonce).as_bytes());
        let hashed: [u8; 32] = digest.finalize().into();
        proof_of_work = hex::encode(hashed);
        nonce += 1;
    }

    println!("Proof of work: {}", proof_of_work);
    let request = format!("{}\nProof of work: {}", buffer, proof_of_work);
    stream.write_all(request.as_bytes())?;
    stream.read_to_string(&mut buffer)?;
    println!("Response: {}", buffer);

    let mut file = File::create("response.txt")?;
    file.write_all(buffer.as_bytes())?;
    println!("Response has been written to file 'response.txt'");

    Ok(())
}
