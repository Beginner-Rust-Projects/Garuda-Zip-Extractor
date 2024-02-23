# Rust ZIP File Extractor

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A simple Rust application for extracting ZIP files.

## Features

- Extracts files from ZIP archives.
- Handles nested folder structures within ZIP files.

## Installation

To build and run the application, you need to have Rust and Cargo installed. If you haven't already, you can install Rust and Cargo by following the instructions at [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-zip-extractor.git
cd rust-zip-extractor

# Build the application
cargo build --release

# Run the application
cargo run --release -- <zip-file-path>
```

Replace `<zip-file-path>` with the path to the ZIP file you want to extract.

## Usage

```bash
cargo run --release -- <zip-file-path>
```

- `<zip-file-path>`: The path to the ZIP file you want to extract.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## Acknowledgments

- This application is based on the `zip` crate for working with ZIP files in Rust.
