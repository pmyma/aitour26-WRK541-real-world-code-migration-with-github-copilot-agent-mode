# Rust App Scaffolding

This project uses [actix-web](https://actix.rs/) and [serde](https://serde.rs/) for building a web server in Rust.

## Structure
- `src/main.rs`: Main entry point with a sample health endpoint.
- `Cargo.toml`: Rust dependencies and project metadata.
- `run.sh`: Script to build and run the project.

## Prerequisites
- [Rust toolchain](https://rustup.rs/) installed (recommended: latest stable)

## How to Run

1. **Install Rust (if not already installed):**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the prompts to complete installation.

2. **Navigate to the project folder:**
   ```bash
   cd rust-app
   ```

3. **Build and run the project:**
   ```bash
   ./run.sh
   ```
   Or, manually:
   ```bash
   cargo build
   cargo run
   ```

4. **Test the health endpoint:**
   Open another terminal and run:
   ```bash
   curl http://127.0.0.1:8080/health
   ```
   You should see:
   ```
   OK
   ```

## Next Steps
- Migrate Python endpoints and logic to Rust using actix-web and serde.
- Add more endpoints and tests as needed.
