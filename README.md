# Programmer Kit (pkit)

Cross-platform Programming Software Manager

![Version](https://img.shields.io/badge/Version-0.0.1-blue)
![Issues](https://img.shields.io/github/issues/dead-projects-inc/pkit-cli)
![License](https://img.shields.io/github/license/dead-projects-inc/pkit-cli)

## Links

- 🌐 **Website**: [pkit.sirblob.co](https://pkit.sirblob.co)
- 📋 **Project Board**: [Trello](https://trello.com/b/wjJqU9ws)
- 🐛 **Issues**: [GitHub Issues](https://github.com/dead-projects-inc/pkit-cli/issues)
- 💬 **Discord Server**: [Join us](https://discord.gg/MHYCWXc83m)
- 📦 **Releases**: [GitHub Releases](https://github.com/dead-projects-inc/pkit-cli/releases)

## Features

- 🔧 **Multi-language Support**: Manage various programming languages and frameworks
- 🌍 **Cross-platform**: Works on Linux, macOS, and Windows
- 📦 **Version Management**: Install, switch, and manage multiple versions
- 🔄 **Easy Updates**: Keep your tools up-to-date with simple commands
- 🛠️ **Environment Management**: Automatic PATH and environment setup
- 📱 **Session Management**: Temporary environment switching for projects

## Installation

### Quick Install (Recommended)

**Unix/Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/dead-projects-inc/pkit-cli/main/scripts/install.sh | bash
```

**Windows (PowerShell):**
```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/dead-projects-inc/pkit-cli/main/scripts/install.ps1" -OutFile "install.ps1"; .\install.ps1
```

### Manual Installation

1. Download the appropriate binary from the [releases page](https://github.com/dead-projects-inc/pkit-cli/releases)
2. Extract and place in your PATH
3. Run the installation script for environment setup

### Custom Installation

You can customize the installation by setting environment variables:

```bash
# Install from a specific version
PKIT_VERSION="v1.0.0" ./install.sh

# Install from a custom repository
PKIT_REPO="myuser/my-pkit-fork" ./install.sh
```

## Usage

### Basic Commands

```bash
# Show help
pkit --help

# List available packages
pkit list

# List installed packages
pkit list --installed

# Install a package
pkit install node 18.0.0

# Set default version
pkit default node 18.0.0

# Switch to a specific version (session-only)
pkit switch node 16.0.0

# Uninstall a package
pkit uninstall node 18.0.0
```

### Environment Management

pkit automatically manages your shell environment:

- **Persistent changes**: `install`, `default`, `uninstall` commands update your shell permanently
- **Session changes**: `switch` command affects only the current session

## Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: x86_64, aarch64 (Apple Silicon)
- **Windows**: x86_64

## Development

### Building from Source

```bash
git clone https://github.com/dead-projects-inc/pkit-cli.git
cd pkit-cli
cargo build
cargo run
```

## Uninstallation

To remove pkit completely:

```bash
# Unix/Linux/macOS
~/.pkit/uninstall.sh

# Windows (PowerShell)
~/.pkit/uninstall.ps1
```

## Contributing

1. Fork the repository
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Roadmap

- [x] Cross-platform CLI tool
- [x] Automatic environment management
- [x] Version switching and management
- [x] Installation scripts for all platforms
- [ ] Package database with popular languages/frameworks
- [ ] Updating installed tools

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE](LICENSE) file for details.