# **Project Veil Timeline**

## **MVP Release in 6 Weeks**

This roadmap outlines the development schedule for **Project Veil**, focusing on delivering an **MVP** within **6 weeks
**. The MVP will include:

- **User & Group Management**
- **Public-Key / DID Authentication**
- **Real-Time WebSocket Chat**
- **End-to-End Encryption (E2EE) using MLS**
- **Basic Terminal UI (TUI) Client**

Advanced privacy features (offline storage, mix networks, PIR, etc.) will follow **post-MVP**.

---

## **Timeline Overview**

| **Step** | **Title**                                           | **Estimated Duration** | **MVP or Post-MVP?** | **Key Milestone**                                 |
|----------|-----------------------------------------------------|------------------------|----------------------|---------------------------------------------------|
| **1**    | **Project Setup & Scaffolding**                     | ~1 week (part-time)    | MVP                  | Basic Rust project, no chat yet                   |
| **2**    | **Basic WS Chat + Basic User Management (no auth)** | ~1 week (part-time)    | MVP                  | Plaintext group chat, user CRUD, TUI foundation   |
| **3**    | **Public-Key / DID Auth**                           | ~1.5 weeks (part-time) | MVP                  | Authenticated chat, TUI login flow                |
| **4**    | **Group Management**                                | ~1 week (part-time)    | MVP                  | Groups, add/remove members, TUI commands          |
| **5**    | **`openmls` E2EE**                                  | ~1.5 weeks (part-time) | **MVP Release**      | End-to-end encrypted group chat (server is relay) |
| **---**  | **MVP Release**                                     | -                      | **MVP**              | **Publish first version**                         |
| **6**    | **Offline Mailbox (Zero-Trust Storage)**            | ~1 week (part-time)    | Post-MVP             | Stored encrypted msgs for offline users           |
| **7**    | **Mix Networks (Hide IP/Network Metadata)**         | ~1 week (part-time)    | Post-MVP             | Traffic routed via Nym (server sees mix IP only)  |
| **8**    | **PIR (Private Message Retrieval)**                 | ~1.5 weeks (part-time) | Post-MVP             | Batch/decoy retrieval to hide which msgs fetched  |
| **9**    | **Final Polishing & Extended TUI Demo**             | ~1 week (part-time)    | Post-MVP             | Full privacy features & advanced demo             |

**Total Time to MVP:** Approximately **6 weeks** (Step 1–5) of part-time work.

---

## **Detailed Development Plan**

### **Week 1: Step 1 – Project Setup & Scaffolding**

- **Tasks**
    1. Create Rust project (`cargo new secure_chat`).
    2. Set up `Cargo.toml` with dependencies (`tokio`, `axum`, etc.).
    3. Verify build (`cargo run` → “Hello, secure chat!”).
- **Milestone:** A skeleton project, no networking or domain logic.

### **Week 2: Step 2 – Basic WS Chat + User Management (No Auth)**

- **Tasks**
    1. Set up WebSocket server (`/ws` endpoint).
    2. Implement user CRUD (`/users` create/list/delete).
    3. Plaintext **broadcast chat** over WebSockets.
    4. Basic **TUI Client**: Connects to WebSocket, sends/receives messages.
- **Milestone:** Plaintext group chat + in-memory user management via TUI.

### **Weeks 3–4: Step 3 – Public-Key / DID Authentication**

- **Tasks**
    1. Server issues **challenge** for login.
    2. Client signs challenge with a **private key**.
    3. Server verifies using stored **public key** or **DID document**.
    4. WebSocket connections require valid authentication.
    5. Update **TUI** to include authentication flow.
- **Milestone:** Authenticated user sessions (only verified users can join chat).

### **Week 5: Step 4 – Group Management**

- **Tasks**
    1. Implement group CRUD: create, list, add/remove users.
    2. Server **routes messages by group** (no more global broadcast).
    3. **TUI commands**: Create/join groups, list members.
- **Milestone:** Users chat **only within their groups**. No encryption yet.

### **Weeks 6–7: Step 5 – `openmls` E2EE (MVP Release)**

- **Tasks**
    1. Integrate `openmls` (Messaging Layer Security).
    2. **Client (TUI):** Encrypt before sending, decrypt incoming.
    3. **Server:** Only relays encrypted messages (zero-trust).
- **Milestone:** End-to-end encrypted **group chat**. The **server cannot read** messages.

### **MVP Release (~6 Weeks In)**

At this point, the system has:

- **User & group management**
- **Authenticated WebSocket chat**
- **End-to-end encryption (MLS)**
- **Basic TUI client**

---

## **Post-MVP Features** (Following Weeks)

### **Step 6: Offline Mailbox (Zero-Trust Storage)**

- **Estimated Time:** ~1 week
- Store encrypted messages for offline users; fetch on reconnect.

### **Step 7: Mix Networks**

- **Estimated Time:** ~1 week
- Route WebSocket traffic through **Nym** to hide real IPs.

### **Step 8: PIR (Private Message Retrieval)**

- **Estimated Time:** ~1.5 weeks
- Hide which messages a user actually fetches using batch retrieval.

### **Step 9: Final Polishing & Extended TUI Demo**

- **Estimated Time:** ~1 week
- Improve **TUI UX**, add **decoy traffic**, final testing.

---

## **Conclusion**

With this revised plan, **Project Veil MVP** will be ready in **6 weeks**. Post-MVP enhancements will further **increase
privacy** and **metadata protection**.

By following this plan, you will build a fully functional **secure group chat with a TUI client**, **public-key/DID
authentication**, **end-to-end encryption**, and **group-based messaging**. Advanced features (mix networks, PIR,
offline storage) will follow afterward.

