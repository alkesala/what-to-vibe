#!/bin/bash

# What to Vibe Installation Script
echo "🎵 Installing What to Vibe..."

# Build the release binary
echo "🔨 Building release binary..."
cargo build --release

# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Copy the binary
echo "📦 Installing to ~/.local/bin/vibe..."
cp target/release/what-to-vibe ~/.local/bin/vibe

# Make it executable
chmod +x ~/.local/bin/vibe

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo "⚠️  ~/.local/bin is not in your PATH"
    echo "Add this line to your ~/.zshrc or ~/.bashrc:"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo "Then run: source ~/.zshrc"
else
    echo "✅ Installation complete!"
    echo "You can now run: vibe --help"
fi 