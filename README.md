# DX - Developer Experience Toolkit

A fast, efficient command-line tool built with Rust to streamline common developer tasks.

## Features

- **Fast & Lightweight**: Built with Rust for optimal performance
- **Multiple Utilities**: JSON manipulation, UUID generation, hashing functions, and more
- **Easy to Use**: Simple, intuitive command-line interface
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/ikok07/dxcli/releases).

### Install with prebuilt script (Linux and MacOS only)

You can automatically install the tool using ```install.sh```

```bash
curl -fsSL https://raw.githubusercontent.com/ikok07/dxcli/master/install.sh | bash
```

### Build from Source

```bash
git clone https://github.com/ikok07/dxcli.git
cd dxcli
cargo build --release
```

The compiled binary will be available at `target/release/dx`.

### Add to PATH

**Important**: Before adding to PATH, move the `dx` executable to a permanent location on your disk (e.g., `~/bin/dx` on Linux/macOS or `C:\Tools\dx` on Windows). Don't add the PATH from temporary locations like your Downloads folder.

#### Windows
1. Type ```Edit the system environment variables``` in the start menu and open it
2. Click on the ```Advanced``` tab 
3. Click ```Environment Variables```
4. Under "User variables", select "Path" and click "Edit"
5. Click "New" and add the path to the directory containing `dx.exe`
6. Click "OK" on all dialogs

#### Linux / MacOS
1. Identify shell
```shell
echo $SHELL
```
2. Navigate to shell config file
- For Zsh: ```nano ~/.zshrc```
- For Bash (Linux): ```nano ~/.bashrc```
- For Bash (MacOS): ```nano ~/.bash_profile```
3. Add this line to the end of the file
```shell
export PATH="/path/to/dx:$PATH"
```
Replace `/path/to/dx` with the actual directory containing the `dx` binary.
4. **Apply the changes**
```bash
source ~/.zshrc    # or ~/.bashrc / ~/.bash_profile
```
Or simply restart your terminal.
5. **Verify installation**
```bash
dx --help
```

## Usage

```bash
dx [OPTIONS] <COMMAND>
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
dx time format      # Reformat timestamp
dx time tz          # List all available timezones
```

#### Text

Text manipulation utilities.

```bash
dx text upper    # Convert to uppercase
dx text lower    # Convert to lowercase
dx text title    # Capitalize text
dx text camel    # Convert to camelCase
dx text pascal    # Convert to PascalCase
dx text snake    # Convert to snake_case
dx text kebab    # Convert to kebab-case

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

This project is licensed under the MIT License

## Support

- Report bugs: [GitHub Issues](https://github.com/ikok07/dxcli/issues)

---

Built with ❤️ using Rust