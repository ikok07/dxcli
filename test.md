# DX - Developer Experience Toolkit

A fast, efficient command-line tool built with Rust to streamline common developer tasks. DX provides utilities for working with JSON, generating UUIDs, hashing, and more—all in one convenient CLI.

## Features

- **Fast & Lightweight**: Built with Rust for optimal performance
- **Multiple Utilities**: JSON manipulation, UUID generation, hashing functions, and more
- **Easy to Use**: Simple, intuitive command-line interface
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/YOUR_USERNAME/dx/releases).

### Install via Cargo

If you have Rust installed, you can install DX directly from crates.io:

```bash
cargo install dx-cli
```

### Build from Source

```bash
git clone https://github.com/YOUR_USERNAME/dx.git
cd dx
cargo build --release
```

The compiled binary will be available at `target/release/dx`.

### Add to PATH

[Add instructions here for adding the binary to system PATH for each OS]

## Usage

```bash
dx [OPTIONS] <COMMAND>
```

### Global Options

```bash
-o, --output <OUTPUT>  Path to file where to save the results
-h, --help             Print help
```

### Available Commands

#### JSON Operations

Format, validate, and manipulate JSON data.

```bash
dx json format      # Format/prettify JSON
dx json minify      # Minify JSON
dx json validate    # Validate JSON syntax
```

#### Encode

Encode data to various formats.

```bash
dx encode base64    # Encode to Base64
dx encode url       # URL encode
dx encode hex       # Encode to hexadecimal
```

#### Decode

Decode data from various formats.

```bash
dx decode base64    # Decode from Base64
dx decode url       # URL decode
dx decode hex       # Decode from hexadecimal
```

#### Hash

Generate and validate cryptographic hashes.

```bash
dx hash md5         # Generate MD5 hash
dx hash sha256      # Generate SHA256 hash
dx hash sha512      # Generate SHA512 hash
dx hash file        # Hash file contents
dx hash verify      # Verify hash against input
```

#### UUID

Generate UUIDs.

```bash
dx uuid v4          # Generate UUID v4 (random)
dx uuid v7          # Generate UUID v7 (time-based)
```

#### Time

Convert and format dates and timestamps.

```bash
dx time now         # Get current timestamp
dx time unix        # Get Unix timestamp
dx time from-unix   # Convert from Unix timestamp
dx time to-unix     # Convert to Unix timestamp
dx time relative    # Calculate relative time
dx time format      # Format timestamp
dx time tz          # Timezone operations
```

#### Text

Text manipulation utilities.

```bash
dx text [subcommand]    # Various text operations
```

#### JWT

Decode and verify JWT tokens.

```bash
dx jwt decode       # Decode JWT token
dx jwt verify       # Verify JWT token
```

#### Regex

Common RegEx operations.

```bash
dx regex test       # Test regex pattern
dx regex match      # Match regex pattern
dx regex replace    # Replace using regex
```

#### Lorem

Generate Lorem Ipsum texts.

```bash
dx lorem words          # Generate random words
dx lorem sentences      # Generate random sentences
dx lorem paragraphs     # Generate random paragraphs
```

## Examples

[Add practical examples of common use cases]

```bash
# Example 1: [Description]
dx [command] [args]

# Example 2: [Description]
dx [command] [args]

# Example 3: [Description]
dx [command] [args]
```

## Configuration

[Add information about any configuration files or environment variables]

DX can be configured via:
- Configuration file: `~/.config/dx/config.toml` (or platform-specific location)
- Environment variables: `DX_[OPTION_NAME]`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- [COMMAND] [OPTIONS]
```

## License

[Add your license here - e.g., MIT, Apache 2.0, GPL-3.0]

## Acknowledgments

[Add any acknowledgments, inspirations, or credits]

## Support

- Report bugs: [GitHub Issues](https://github.com/YOUR_USERNAME/dx/issues)
- Documentation: [Add link to docs if applicable]
- Discussions: [Add link to discussions/community if applicable]

---

Built with ❤️ using Rust