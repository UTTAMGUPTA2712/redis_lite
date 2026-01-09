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

*To be added as the implementation progresses.*

---
*Inspired by the official [Redis Protocol Specification](https://redis.io/docs/latest/develop/reference/protocol-spec/)*
