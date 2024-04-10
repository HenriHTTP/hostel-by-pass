# Hostel By Pass

Hostel By Pass is a microservice written in Rust and Axum framework, designed to provide various useful features for managing room reservations. The microservice uses MongoDB as its database.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](#)
[![MongoDB](https://img.shields.io/badge/MongoDB-4EA94B?style=for-the-badge&logo=mongodb&logoColor=white)](#)

## Install the Dependencies

Before running Hostel By Pass, ensure you have Rust installed on your system.

### Rust

You can install Rust by following the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).

### Docker Compose

If you already have a `docker-compose.yml` file in your project directory:

1. Open a terminal.
2. Navigate to your project directory where the `docker-compose.yml` file is located.
3. Run the following command to start the MongoDB container:

# How start
### Docker compose
Open a terminal and write this command:
```bash
docker-compose up -d
```
### Rust app

Open a terminal and write this command:
```bash
cargo build
```
before write this command: 
```bash
cargo run
```
