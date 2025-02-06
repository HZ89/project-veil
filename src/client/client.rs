use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;
use crate::protocol::protocol::ProtocolMessage;
use ed25519_dalek::SigningKey; // updated for ed25519-dalek v2.1.1
use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey}; // using static_secrets feature
use rand_core::OsRng;
use crate::protocol::protocol::crypto;
use std::error::Error;
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run_client(addr: &str, username: &str) -> Result<(), Box<dyn Error>> {
    // Connect to the server.
    let mut socket = TcpStream::connect(addr).await?;
    println!("Connected to server at {}", addr);

    // Generate the client's signing key.
    let mut csprng = OsRng{};
    let client_signing = SigningKey::generate(&mut csprng);

    // Generate the client's encryption keypair using StaticSecret.
    let client_encryption = StaticSecret::new(&mut csprng);
    let client_encryption_pub = X25519PublicKey::from(&client_encryption);

    // Build the join request with the client's public keys.
    let join_request = ProtocolMessage::JoinRequest {
        username: username.to_string(),
        encryption_pub: client_encryption_pub.as_bytes().to_vec(),
        signing_pub: client_signing.verifying_key().to_bytes().to_vec(),
    };
    let join_request_bytes = serde_json::to_vec(&join_request)?;
    socket.write_all(&join_request_bytes).await?;

    // Read the join response from the server.
    let mut buf = vec![0u8; 1024];
    let n = socket.read(&mut buf).await?;
    if n == 0 {
        println!("Server closed connection");
        return Ok(());
    }
    let response: ProtocolMessage = serde_json::from_slice(&buf[..n])?;
    let group_key = match response {
        ProtocolMessage::JoinResponse { encrypted_group_key, admin_signing_pub: _, admin_encryption_pub } => {
            // Convert the admin's encryption public key (from Vec<u8>) to [u8; 32].
            let admin_encryption_pub = X25519PublicKey::from(
                <[u8; 32]>::try_from(admin_encryption_pub.as_slice())
                    .expect("Expected 32 bytes for admin encryption public key")
            );
            // Decrypt the group key using the client's encryption key.
            crypto::decrypt_group_key(&encrypted_group_key, client_encryption, &admin_encryption_pub)
        },
        _ => {
            println!("Unexpected response from server.");
            return Ok(());
        }
    };

    println!("Successfully joined group. Group key: {:02x?}", group_key);

    // Demonstration: send a chat message.
    let msg = "Hello, this is a test message.";
    let msg_id = format!("{}", SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis());
    let chat_message = ProtocolMessage::ChatMessage {
        username: username.to_string(),
        msg_id,
        // In a full implementation, you would encrypt this message using the shared group key.
        ciphertext: msg.as_bytes().to_vec(),
        // In a full implementation, sign the message.
        signature: vec![],
    };
    let chat_message_bytes = serde_json::to_vec(&chat_message)?;
    socket.write_all(&chat_message_bytes).await?;
    println!("Chat message sent.");

    Ok(())
}