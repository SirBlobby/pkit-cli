#!/bin/bash
# Programmer Kit (pkit) Installation Script
# This script downloads the latest pkit executable from GitHub releases and installs it to ~/.pkit
# Can be run from anywhere - no build required

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

command -v curl >/dev/null 2>&1 || { print_error "curl is required but not installed."; exit 1; }
command -v tar >/dev/null 2>&1 || { print_error "tar is required but not installed."; exit 1; }

GITHUB_REPO="${PKIT_REPO:-dead-projects-inc/pkit-cli}"
PKIT_VERSION="${PKIT_VERSION:-latest}"
INSTALL_DIR="$HOME/.pkit"
BIN_DIR="$INSTALL_DIR/bin"
TEMP_DIR=$(mktemp -d)

cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

print_status "Installing pkit from $GITHUB_REPO (version: $PKIT_VERSION)"

ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case $ARCH in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) print_error "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case $OS in
    linux) OS="linux" ;;
    darwin) OS="macos" ;;
    *) print_error "Unsupported operating system: $OS"; exit 1 ;;
esac

BINARY_NAME="pkit-${OS}-${ARCH}"
print_status "Detected system: $OS $ARCH"

if [ "$PKIT_VERSION" = "latest" ]; then
    print_status "Fetching latest release..."
    RELEASE_URL="https://api.github.com/repos/$GITHUB_REPO/releases/latest"
    DOWNLOAD_URL=$(curl -s "$RELEASE_URL" | grep -o "https://github.com/$GITHUB_REPO/releases/download/[^\"]*$BINARY_NAME[^\"]*" | head -1)
else
    DOWNLOAD_URL="https://github.com/$GITHUB_REPO/releases/download/$PKIT_VERSION/$BINARY_NAME"
fi

if [ -z "$DOWNLOAD_URL" ]; then
    print_error "Could not find download URL for $BINARY_NAME"
    print_error "Check if a release exists at: https://github.com/$GITHUB_REPO/releases"
    exit 1
fi

print_status "Download URL: $DOWNLOAD_URL"

print_status "Downloading pkit..."
DOWNLOAD_PATH="$TEMP_DIR/pkit"
if ! curl -L -o "$DOWNLOAD_PATH" "$DOWNLOAD_URL"; then
    print_error "Failed to download pkit"
    exit 1
fi

if file "$DOWNLOAD_PATH" | grep -q "gzip\|tar"; then
    print_status "Extracting..."
    cd "$TEMP_DIR"
    tar -xf "$DOWNLOAD_PATH"
    BINARY_PATH=$(find "$TEMP_DIR" -name "pkit" -type f | head -1)
    if [ -z "$BINARY_PATH" ]; then
        print_error "Could not find pkit binary in extracted files"
        exit 1
    fi
else
    BINARY_PATH="$DOWNLOAD_PATH"
fi

chmod +x "$BINARY_PATH"

CURRENT_VERSION=""
if [ -f "$BIN_DIR/pkit" ]; then
    print_status "Found existing installation"
    if "$BIN_DIR/pkit" --version >/dev/null 2>&1; then
        CURRENT_VERSION=$("$BIN_DIR/pkit" --version 2>&1)
        print_status "Current version: $CURRENT_VERSION"
    fi
    
    if [ "$PKIT_VERSION" = "latest" ]; then
        LATEST_VERSION=$(curl -s "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)
        if [ -n "$LATEST_VERSION" ] && [[ "$CURRENT_VERSION" == *"$LATEST_VERSION"* ]]; then
            print_success "Already up to date ($CURRENT_VERSION)"
            exit 0
        fi
    fi
fi

print_status "Installing to $BIN_DIR"
mkdir -p "$BIN_DIR"

if [ -f "$BIN_DIR/pkit" ]; then
    cp "$BIN_DIR/pkit" "$BIN_DIR/pkit.backup" 2>/dev/null || true
fi

cp "$BINARY_PATH" "$BIN_DIR/pkit"
chmod +x "$BIN_DIR/pkit"

if ! "$BIN_DIR/pkit" --version >/dev/null 2>&1; then
    if [ -f "$BIN_DIR/pkit.backup" ]; then
        mv "$BIN_DIR/pkit.backup" "$BIN_DIR/pkit"
        print_error "New binary failed, restored backup"
        exit 1
    fi
    print_warning "Binary test failed"
fi

rm -f "$BIN_DIR/pkit.backup" 2>/dev/null || true

setup_shell_env() {
    local shell_file="$1"
    local shell_name=$(basename "$shell_file")
    
    if [ ! -f "$shell_file" ]; then
        touch "$shell_file"
        print_status "Created $shell_name"
    fi
    
    cp "$shell_file" "${shell_file}.pkit-backup" 2>/dev/null || true
    
    sed -i.tmp '/# pkit-cli-env-start/,/# pkit-cli-env-end/d' "$shell_file" 2>/dev/null || true
    sed -i.tmp '/^pkit() {$/,/^}$/d' "$shell_file" 2>/dev/null || true
    sed -i.tmp '/# Added by pkit installer/d' "$shell_file" 2>/dev/null || true
    sed -i.tmp '/export PATH=.*\.pkit\/bin/d' "$shell_file" 2>/dev/null || true
    rm -f "${shell_file}.tmp" 2>/dev/null || true
    
    cat >> "$shell_file" << 'SHELL_CONFIG'

# pkit-cli-env-start
export PKIT_HOME="$HOME/.pkit"
export PATH="$HOME/.pkit/bin:$PATH"
[[ -s "$PKIT_HOME/pkit_env.sh" ]] && source "$PKIT_HOME/pkit_env.sh"
# pkit-cli-env-end

pkit() {
  command pkit "$@"

  local env_file="${PKIT_HOME:-$HOME/.pkit}/pkit_env.sh"
  local session_env_file="${PKIT_HOME:-$HOME/.pkit}/pkit_session_env.sh"

  if [[ -f "$env_file" && -r "$env_file" ]]; then
    case "$1" in
      default|install|uninstall|path)
        source "$env_file" && echo "pkit environment reloaded."
        ;;
      switch)
        source "$env_file"
        if [[ -f "$session_env_file" && -r "$session_env_file" ]]; then
          source "$session_env_file" && echo "pkit session environment loaded."
        fi
        ;;
    esac
  elif [[ "$1" == "default" || "$1" == "install" || "$1" == "uninstall" || "$1" == "switch" ]]; then
    echo "Warning: Environment file not found at $env_file" >&2
  fi
}
SHELL_CONFIG
    
    print_success "Configured $shell_name"
}

print_status "Configuring shell environment..."
setup_shell_env "$HOME/.bashrc"

for shell_file in "$HOME/.zshrc" "$HOME/.profile"; do
    if [ -f "$shell_file" ]; then
        setup_shell_env "$shell_file"
    fi
done

cat > "$INSTALL_DIR/uninstall.sh" << 'UNINSTALL_SCRIPT'
#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Uninstalling pkit...${NC}"

for shell_file in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile" "$HOME/.bash_profile"; do
    if [ -f "$shell_file" ]; then
        if grep -q "# pkit-cli-env-start" "$shell_file" 2>/dev/null; then
            sed -i.bak '/# pkit-cli-env-start/,/# pkit-cli-env-end/d' "$shell_file" 2>/dev/null || true
            sed -i.bak '/^pkit() {$/,/^}$/d' "$shell_file" 2>/dev/null || true
            sed -i.bak '/# Added by pkit installer/d' "$shell_file" 2>/dev/null || true
            sed -i.bak '/export PATH=.*\.pkit\/bin/d' "$shell_file" 2>/dev/null || true
            rm -f "${shell_file}.bak" 2>/dev/null || true
            echo -e "${GREEN}Cleaned $(basename "$shell_file")${NC}"
        fi
    fi
done

if [ -d "$HOME/.pkit" ]; then
    rm -rf "$HOME/.pkit"
    echo -e "${GREEN}Removed ~/.pkit${NC}"
fi

echo -e "${GREEN}Uninstall complete${NC}"
echo -e "${YELLOW}Please restart your shell${NC}"
UNINSTALL_SCRIPT

chmod +x "$INSTALL_DIR/uninstall.sh"

VERSION_OUTPUT=$("$BIN_DIR/pkit" --version 2>&1 || echo "unknown")

if [ -n "$CURRENT_VERSION" ]; then
    print_success "Updated pkit successfully!"
    print_status "Previous: $CURRENT_VERSION"
    print_status "Current: $VERSION_OUTPUT"
else
    print_success "Installed pkit successfully!"
    print_status "Version: $VERSION_OUTPUT"
fi

print_status "Location: $BIN_DIR/pkit"
print_status "Uninstall: $INSTALL_DIR/uninstall.sh"
print_warning "Restart your shell or run: source ~/.bashrc"
print_status "Test with: pkit --help"
