# **Project Veil**

## **Overview**

Project Veil is a privacy-focused, end-to-end encrypted (E2EE) group chat application that ensures secure and anonymous
communication. Built with Rust, it leverages **WebSockets**, **Messaging Layer Security (MLS)**, and **Decentralized
Identifiers (DIDs)** for authentication and encryption. The application features a **Terminal User Interface (TUI)** and
is designed for zero-trust environments, ensuring that even the server cannot read message contents.

## **Key Features**

- **Secure Group Chat**: Real-time messaging over WebSockets.
- **End-to-End Encryption (E2EE)**: Utilizes `openmls` for secure communication.
- **Public-Key / DID Authentication**: No passwords, only cryptographic identities.
- **Zero-Trust Message Storage**: The server stores only encrypted data.
- **Offline Message Support**: Retrieve undelivered messages securely.
- **Privacy Enhancements (Post-MVP)**:
    - Mix Networks for IP anonymity.
    - Private Information Retrieval (PIR) for metadata protection.
    - Decoy traffic for enhanced privacy.

## **Technology Stack**

- **Programming Language**: Rust
- **Networking**: WebSockets (`axum` or `tokio-tungstenite`)
- **Encryption**: Messaging Layer Security (`openmls`)
- **Authentication**: Public-Key Crypto (Ed25519, secp256k1) & DIDs
- **Database**: In-memory storage (future: optional persistent DB)
- **Privacy Enhancements**: Mix Networks (`nym`), PIR, decoy messages

## **Getting Started**

### **Prerequisites**

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (latest stable version)
- Cargo (Rust’s package manager)

### **Installation**

1. Clone the repository:
   ```sh
   git clone https://github.com/hz89/project-veil.git
   cd project-veil
   ```
2. Build the project:
   ```sh
   cargo build --release
   ```

### **Usage**

#### **Running the Server**

```sh
cargo run --bin server
```

#### **Running the TUI Client**

```sh
cargo run --bin client
```

#### **Interacting with the Chat**

- **Create a user**
- **Authenticate using a private key**
- **Join or create a group**
- **Send & receive encrypted messages**

## **Development Roadmap**

### **MVP Features (First 6 Weeks)**

✅ WebSocket-based real-time messaging  
✅ Public-Key / DID authentication  
✅ End-to-End Encryption (E2EE) with `openmls`  
✅ Basic TUI client  
✅ Group Management (create, list, add/remove members)

### **Post-MVP Enhancements**

🔲 Offline Message Queue (Zero-Trust Storage)  
🔲 Mix Networks for Traffic Anonymity  
🔲 PIR for Private Message Retrieval  
🔲 Decoy Traffic to Obfuscate User Activity

## **License**

This project is licensed under the **MIT License**. See `LICENSE` for details.

## **Contributing**

Contributions are welcome! To get started:

1. Fork the project.
2. Create a feature branch (`git checkout -b feature-name`).
3. Commit your changes (`git commit -m 'Add feature XYZ'`).
4. Push to your fork and create a Pull Request.

## **Contact & Support**

For any issues or suggestions, please open an **issue** on GitHub.

---

**Project Veil** – Secure, private, and anonymous communication for everyone.

