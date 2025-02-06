use crate::protocol::protocol::crypto;
use crate::protocol::protocol::ProtocolMessage;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::{rng, RngCore};
use rand_core::OsRng;
use serde_json;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret}; // Ensure "static_secrets" feature is enabled

/// Server state holds the admin signing key, a stored 32-byte seed for the encryption key,
/// the corresponding public key, the group key (K0), and a list of connected clients.
pub struct ServerState {
    // Admin signing key (for message signing).
    pub admin_signing: SigningKey,
    // Instead of storing a non‑cloneable secret, store a 32-byte seed.
    pub admin_encryption_seed: [u8; 32],
    // The public key derived from the admin encryption key.
    pub admin_encryption_pub: X25519PublicKey,
    // The group key (K0).
    pub group_key: Vec<u8>,
    // For demonstration, a map of connected clients.
    pub clients: HashMap<String, TcpStream>,
}

impl ServerState {
    pub fn new() -> Self {
        // Create an OS RNG for key generation.
        let mut csprng = OsRng {};
        // Generate the admin signing key.
        let admin_signing = SigningKey::generate(&mut csprng);

        // Generate a random 32-byte seed for the encryption key.
        let mut seed = [0u8; 32];
        rng().fill_bytes(&mut seed);
        // Create a static secret (admin encryption key) from the seed.
        let admin_encryption = StaticSecret::from(seed);
        // Derive the corresponding public key.
        let admin_encryption_pub = X25519PublicKey::from(&admin_encryption);

        // Generate the initial group key (K0) as 32 random bytes.
        let mut group_key = vec![0u8; 32];
        rng().fill_bytes(&mut group_key);

        ServerState {
            admin_signing,
            admin_encryption_seed: seed,
            admin_encryption_pub,
            group_key,
            clients: HashMap::new(),
        }
    }
}

/// Runs the server by binding to the given address and handling incoming connections.
pub async fn run_server(addr: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    let state = Arc::new(Mutex::new(ServerState::new()));

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {:?}", addr);
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(&mut socket, state).await {
                println!("Error handling client: {:?}", e);
            }
        });
    }
}

async fn handle_client(
    socket: &mut TcpStream,
    state: Arc<Mutex<ServerState>>,
) -> Result<(), Box<dyn Error>> {
    // Read data from the socket.
    let mut buf = vec![0u8; 1024];
    let n = socket.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    // Deserialize the received JSON into a ProtocolMessage.
    let msg: ProtocolMessage = serde_json::from_slice(&buf[..n])?;
    match msg {
        ProtocolMessage::JoinRequest {
            username,
            encryption_pub,
            signing_pub,
        } => {
            println!("Join request from {}", username);

            // Convert the client's encryption public key (Vec<u8>) into a fixed-size [u8; 32].
            let client_encryption_pub = X25519PublicKey::from(
                <[u8; 32]>::try_from(encryption_pub.as_slice())
                    .expect("Expected 32 bytes for client encryption public key"),
            );

            // Convert the client's signing public key into a [u8; 32] array.
            let client_signing_pub_array: [u8; 32] = signing_pub
                .as_slice()
                .try_into()
                .expect("Expected 32 bytes for client signing public key");
            // Create a VerifyingKey from the fixed-size array.
            let _client_signing_pub = VerifyingKey::from_bytes(&client_signing_pub_array)?;

            // Extract required state values in a separate block so that the mutex guard is dropped before the await.
            let (encrypted_group_key, admin_signing_pub, admin_encryption_pub) = {
                let state_guard = state.lock().unwrap();
                // Recreate the admin encryption key from the stored seed.
                let admin_secret = StaticSecret::from(state_guard.admin_encryption_seed);
                let encrypted_group_key = crypto::encrypt_group_key(
                    &state_guard.group_key,
                    admin_secret,
                    &client_encryption_pub,
                );
                let admin_signing_pub = state_guard
                    .admin_signing
                    .verifying_key()
                    .to_bytes()
                    .to_vec();
                let admin_encryption_pub = state_guard.admin_encryption_pub.as_bytes().to_vec();
                (encrypted_group_key, admin_signing_pub, admin_encryption_pub)
            };

            let response = ProtocolMessage::JoinResponse {
                encrypted_group_key,
                admin_signing_pub,
                admin_encryption_pub,
            };

            let response_bytes = serde_json::to_vec(&response)?;
            // Write the response asynchronously.
            socket.write_all(&response_bytes).await?;
        }
        ProtocolMessage::ChatMessage {
            username,
            msg_id,
            ciphertext,
            signature,
        } => {
            println!("Chat message from {} (msg_id: {})", username, msg_id);
            // In a complete implementation, relay this message to other clients.
        }
        _ => {
            println!("Received unsupported message type.");
        }
    }
    Ok(())
}
