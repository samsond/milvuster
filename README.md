# Milvuster

Milvuster is a Rust client for interacting with the Milvus vector database. This project includes functionality to connect to a Milvus server, create databases, and list databases. It also includes a Docker Compose setup for running Milvus, etcd, and Minio services.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Docker (https://docs.docker.com/get-docker/)
- Docker Compose (https://docs.docker.com/compose/install/)

## Getting Started

### Clone the Repository

```sh
git clone https://github.com/yourusername/milvuster.git
cd milvuster
```

## Set Up Docker Compose
The docker-compose.yml file is configured to run Milvus, etcd, and Minio services. To start the services, run:

```sh
docker-compose up -d
```

## Build and Run the Project
To build the project, run:
```sh
cargo build
cargo run
```

This will execute the main function in main.rs, which connects to the Milvus server, creates a new database, and lists all databases.

## Project Structure

* `main.rs:` The main entry point of the application.
* `client.rs:` Contains functions to connect to Milvus, create databases, and list databases.
* `proto`: Contains the generated Rust code from the Milvus proto files.
* `docker-compose.yml`: Docker Compose configuration for running Milvus, etcd, and Minio services.

## License
TODO
