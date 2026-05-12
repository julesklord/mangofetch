# 🛠️ P2P File Sharing

MangoFetch isn't just for downloading from the web. It features a high-speed, direct **P2P (Peer-to-Peer)** file transfer system built on top of Rust's networking stack. This allows you to send files directly to another MangoFetch user without relying on a central server.

## 🚀 Sending a File

To send a file to a friend, use the `send` command:

```bash
mangofetch send path/to/your/file.zip
```

### What happens next?
1.  MangoFetch generates a **Unique Code** (e.g., `tropical-mango-82`).
2.  Your computer starts a secure listener.
3.  Share the code with your recipient.

---

## 📥 Receiving a File

To receive a file, the recipient simply runs the `download` command with the unique code:

```bash
mangofetch download tropical-mango-82
```

MangoFetch will automatically detect that it's a P2P transfer, connect to your machine, and stream the file securely.

---

## 🔒 Security & Privacy

*   **Direct Connection**: Files are transferred directly from one computer to another. No data is stored on our servers.
*   **Checksum Verification**: Every P2P transfer includes automatic SHA-256 checksum verification to ensure the file arrived exactly as it was sent.
*   **Encrypted Streams**: The transfer is encrypted to prevent eavesdropping on public networks.

---

## ⚡ Performance

The P2P engine utilizes **multi-threaded streaming** and **zero-copy buffering**, allowing you to saturate your upload speed even when transferring multi-gigabyte files.
