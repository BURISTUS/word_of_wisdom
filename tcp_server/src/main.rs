use rand::{rngs::OsRng, Rng};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, net::SocketAddr, time::Instant};
use tcp_server::{config::get_config, errors::GeneralErrors};
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<(), GeneralErrors> {
    let configuration = get_config().map_err(|_| GeneralErrors::ReadConfigError)?;
    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application_config.host, configuration.application_config.port
    ))
    .await
    .map_err(|_| GeneralErrors::TcpListenerError)?;
    
    let mut client_requests: HashMap<SocketAddr, Instant> = HashMap::new();
    let request_timeout = Duration::from_secs(configuration.settings_config.request_timeout);
    let max_connections = configuration.settings_config.request_timeout;
    let target_difficulty = configuration.settings_config.target_difficulty;
    let mut connection_count = 0;
    
    while let Ok((stream, addr)) = listener.accept().await {
        if connection_count >= max_connections {
            let (_, mut writer) = split(stream);
            match writer.write("Too many connections".as_bytes()).await {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Failed to write to the stream");
                    continue;
                }
            }
            continue;
        }
        
        let current_time = Instant::now();
        
        if let Some(last_request_time) = client_requests.get(&addr) {
            if current_time.duration_since(*last_request_time) < request_timeout {
                let (_, mut writer) = split(stream);
                match writer.write("Too many requests".as_bytes()).await {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!("Failed to write to the stream");
                        continue;
                    }
                }
                continue;
            }
        }
        
        connection_count += 1;
        client_requests.insert(addr, current_time);
        
        tokio::spawn(async move {
            handle_client(stream, target_difficulty).await;
        });
        
        connection_count -= 1;
    }

    Ok(())
}

async fn handle_client(stream: TcpStream, target_difficulty: usize) {
    let (mut reader, mut writer) = split(stream);
    let mut buffer = [0; 1024];
    
    match reader.read(&mut buffer).await {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received request: {}", request);

            let mut nonce = [0u8; 8];
            let mut rng: OsRng = OsRng::default();
            rng.fill(&mut nonce);
            let mut nonce = u64::from_le_bytes(nonce);
            let digest = Sha256::new();
            let mut proof_of_work = String::new();
            let zero_string = "0".repeat(target_difficulty);

            while !proof_of_work.starts_with(&zero_string) {
                let mut digest = digest.clone(); // reset the digest before updating it
                digest.update(format!("{}{}", request, nonce).as_bytes());
                let hashed: [u8; 32] = digest.finalize().into();
                proof_of_work = hex::encode(hashed);
                nonce += 1;
            }

            println!("Proof of work: {}", proof_of_work);

            let wisdom = get_wisdom();
            let response = format!("{}\nProof of work: {}", wisdom, proof_of_work);
            match writer.write(response.as_bytes()).await {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Failed to write to the stream")
                }
            }
        }
        Err(e) => {
            println!("Error reading from stream: {}", e);
        }
    }
}

fn get_wisdom() -> String {
    let wisdom: [&str; 3] = [
        "A closed mouth catches no flies.",
        "An apple a day keeps the doctor away.",
        "Early to bed and early to rise makes a man healthy, wealthy, and wise.",
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..wisdom.len());
    wisdom[index].to_string()
}
