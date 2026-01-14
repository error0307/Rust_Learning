# 🦀 Rust Learning - Practical Rust Programming Learning Environment

This repository provides a comprehensive learning environment for Rust programming, from fundamentals to advanced topics. It includes various sample projects organized by learning themes and is available with a Docker environment for easy setup.

## 📁 Project Structure

```
Rust_Learning/
├── README.md                 # This file
├── DOCKER_README.md          # Docker usage guide
├── Dockerfile                # Docker environment definition
├── docker-compose.yml        # Docker Compose configuration
├── copy_to_running_env.sh    # Source code execution script
│
├── running_env/              # Integrated execution environment
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       └── main.rs
│
└── sources/                  # Learning sample code
    ├── Args_BuilderPattern/          # CLI argument parsing (Builder pattern)
    ├── Args_DerivePattern/           # CLI argument parsing (Derive pattern)
    ├── AsyncMove/                    # async/await basics
    ├── Async_Await_Retly/            # Asynchronous retry handling
    ├── ErrorHandling/                # Error handling
    ├── InstantWebServer/             # Web server with Tokio
    ├── MultiThread_Program_with_SharedMemory/     # Shared memory with Arc/Mutex
    ├── MultiThread_Program_with_SharedMemory_2/   # Thread control with flags
    ├── MultiThread_with_MessagePassing/           # Message passing via channels
    ├── MultiThread_with_MessagePassing2/          # Bidirectional message passing
    ├── Polling_Retly/                # Learning Future::poll
    ├── Polling_Retry_Async/          # Async sleep implementation
    └── Trait_Impl/                   # Trait implementation
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.91.0+
- Cargo

or

- Docker 27.5+

### Running on Local Environment

#### 1. Copy Source Code

```bash
cd /path/to/Rust_Learning

# Copy ErrorHandling to the execution environment
./copy_to_running_env.sh sources/ErrorHandling
```

#### 2. Run Program

```bash
cd running_env
cargo run
```

#### 3. Expected Output

```
Hello, world!
Error: An error occurred during calculation
```

### Running Multiple Samples Sequentially

```bash
cd running_env

# Run ErrorHandling
./copy_to_running_env.sh ../sources/ErrorHandling && cargo run

# Run InstantWebServer (listens on port 8080)
./copy_to_running_env.sh ../sources/InstantWebServer && cargo run
```

## 🐳 Running with Docker

### Build

```bash
docker build -t rust-learning-env .
```

### Start Container (Interactive Mode)

```bash
docker run -it rust-learning-env bash
```

### Run Program Inside Container

```bash
cd /app
./copy_to_running_env.sh sources/ErrorHandling
cd running_env
cargo run
```

For more detailed Docker usage, please refer to [DOCKER_README.md](DOCKER_README.md).

## 📚 Learning Topics

| Folder | Learning Content | Difficulty |
|--------|------------------|-----------|
| ErrorHandling | Result type and error handling | ⭐ |
| Trait_Impl | Trait implementation and usage | ⭐⭐ |
| Args_BuilderPattern | CLI argument parsing with clap (Builder) | ⭐⭐ |
| Args_DerivePattern | CLI argument parsing with clap (Derive) | ⭐⭐ |
| MultiThread_Program_with_SharedMemory | Shared memory with Arc and Mutex | ⭐⭐ |
| MultiThread_Program_with_SharedMemory_2 | Thread control with flags | ⭐⭐ |
| MultiThread_with_MessagePassing | Message passing via channels | ⭐⭐ |
| MultiThread_with_MessagePassing2 | Bidirectional message passing | ⭐⭐⭐ |
| AsyncMove | Async/await basics | ⭐⭐ |
| Async_Await_Retly | Asynchronous retry logic | ⭐⭐⭐ |
| Polling_Retly | Manual Future::poll implementation | ⭐⭐⭐ |
| Polling_Retry_Async | Tokio::time::sleep usage | ⭐⭐⭐ |
| InstantWebServer | Web server with Tokio | ⭐⭐⭐ |

## 🔧 Dependencies

- **tokio** - Asynchronous runtime and network processing
- **clap** - Command-line argument parser
- **futures** - Future trait and utilities

## 💡 How to Run Each Project

### Basic Workflow

```bash
cd running_env

# Copy desired project
../copy_to_running_env.sh ../sources/<ProjectName>

# Build and run
cargo run

# Or run optimized build
cargo run --release
```

## 📝 Script Usage

### copy_to_running_env.sh

Copies source code files (`.rs`) from the specified project to running_env/src directory.

**Usage:**
```bash
./copy_to_running_env.sh <path/to/sources/ProjectName>
```

**Examples:**
```bash
./copy_to_running_env.sh ../sources/ErrorHandling
./copy_to_running_env.sh ../sources/InstantWebServer
```

## ✅ Verified Environments

- **OS**: Linux (Ubuntu 22.04)
- **Rust**: 1.91.1
- **Cargo**: 1.91.1
- **Docker**: 27.5.1

For Windows and macOS environments, please use Docker.

## 🎯 Learning Approach

1. Start with projects of lower difficulty
2. Read and understand the source code of each project
3. Copy with copy_to_running_env.sh and run
4. Verify output results and experiment by modifying the code
5. Create advanced examples by combining multiple projects

## 🤝 Troubleshooting

### Build Error

```bash
cargo clean
cargo build
```

### Multiple Files in running_env/src

The copy_to_running_env.sh script copies only `.rs` files. Files from previous projects may remain. Please delete them manually if necessary.

### Permission Error with Docker

```bash
sudo docker run -it rust-learning-env bash
```

## 📖 References

- [Rust Official Documentation](https://doc.rust-lang.org/)
- [Tokio Tutorial](https://tokio.rs/)
- [clap documentation](https://docs.rs/clap/)

## 📄 License

MIT License

## 👤 Author

Keisuke Ota