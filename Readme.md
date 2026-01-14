# Redis-Lite: A Rust Learning Journey

This repository documents my journey of learning **Rust** by building a lightweight implementation of **Redis** side-by-side. The goal is not just to replicate Redis features, but to understand the underlying concepts while mastering Rust's ownership model, concurrency, and networking capabilities.

## About The Project

I am implementing a networked Key-Value store that supports basic commands (like `GET`, `SET`, `DELETE`) over TCP. This project serves as a practical playground to deeper understand:
- **Rust Systems Programming**: Handling memory safety and low-level control.
- **Asynchronous Programming**: Leveraging the [Tokio](https://tokio.rs/) runtime for handling multiple concurrent connections.
- **Redis Internals**: implementing the [RESP (Redis Serialization Protocol)](https://redis.io/docs/latest/develop/reference/protocol-spec/) from scratch.

## Key Concepts & Learnings

- **TCP Networking**: establishing and managing raw TCP connections.
- **Protocol Parsing**: Writing a custom parser for the RESP protocol.
- **Concurrency**: Using Tokio's async/await pattern to handle high throughput.
- **Data Structures**: Managing shared state safely across threads using Rust's synchronization primitives.

## Usage

To start the Redis-Lite server, run:

```bash
cargo run --release
```

The server listens on `127.0.0.1:6379` by default.

### Commands

Connect to the server using `redis-cli`, `nc`, or a custom TCP client. Note that some commands currently require strict argument counts.

| Command | Usage | Description |
|---------|-------|-------------|
| **PING** | `PING [message]` | Returns `PONG` or `PONG <message>`. |
| **GET** | `GET <key>` | Retrieves the value of `<key>`. |
| **SET** | `SET <key> <value> <ttl>` | Sets `<key>` to `<value>` with an expiration of `<ttl>` seconds. **TTL is mandatory.** |
| **DELETE** | `DELETE <key> <ignored>` | Deletes `<key>`. **Requires an extra dummy argument.** |

### Features

- **Persistence**: Run with `--persist` to enable simple file-based persistence using `db.txt` and `expires.txt`.
  ```bash
  cargo run -- --persist
  ```
- **Logging**: Request logs are saved to `logs/logger-<timestamp>.txt`.
- **Expiration**: Keys automatically expire based on the TTL provided during `SET`.

---
*Inspired by the official [Redis Protocol Specification](https://redis.io/docs/latest/develop/reference/protocol-spec/)*
