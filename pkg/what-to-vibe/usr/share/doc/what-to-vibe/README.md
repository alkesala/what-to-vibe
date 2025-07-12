# What to Vibe

A Rust CLI application that provides themed responses based on your current mood. Get personalized themes, music recommendations, tech stack suggestions, and motivational mottos.

## Installation

### From AUR (Arch Linux)

```bash
# Using yay (recommended)
yay -S what-to-vibe

# Using paru
paru -S what-to-vibe

# Using manual AUR helper
git clone https://aur.archlinux.org/what-to-vibe.git
cd what-to-vibe
makepkg -si
```

### Manual Installation

```bash
# Install Rust (if not already installed)
sudo pacman -S rust  # For Arch Linux
# or visit https://rustup.rs/ for other systems

# Clone and install
git clone https://github.com/yourusername/what-to-vibe.git
cd what-to-vibe
./install.sh

# Or build and install manually
cargo build --release
sudo cp target/release/what-to-vibe /usr/local/bin/vibe
```

## Usage

```bash
vibe <MOOD>
```

### Available Moods

- **focus** - For deep work and concentration
- **chaotic** - For high-energy, experimental coding
- **sadboi** - For introspective, melancholic vibes
- **energetic** - For high-energy, fast-paced development
- **chill** - For relaxed, steady coding sessions
- **creative** - For artistic and experimental projects
- **productive** - For corporate and efficient workflows
- **nostalgic** - For retro and classic computing
- **adventurous** - For cutting-edge and experimental tech
- **zen** - For minimal and mindful programming

### Examples

```bash
# Get focused vibes
vibe focus

# Embrace the chaos
vibe chaotic

# Code through the feels
vibe sadboi

# Random mood selection
vibe

# Interactive mode
vibe --interactive

# Pomodoro timer (25 minutes)
vibe --timer

# Custom timer with specific mood
vibe --timer 5 --mood energetic

# Show ASCII art
vibe focus --ascii

# GODMODE
vibe --godmode
```

## Features

- **Themed responses** - Each mood has a unique color theme
- **Music recommendations** - Curated playlists for each mood
- **Tech stack suggestions** - Programming languages and tools that match the vibe
- **Motivational mottos** - Inspirational quotes to keep you going
- **Colored output** - Beautiful terminal formatting with colors
- **Interactive mode** - Navigate moods with a terminal UI
- **Pomodoro timer** - Built-in productivity timer with mood themes
- **ASCII art** - Visual representations for each mood
- **GODMODE** - Over-the-top elite hacker experience

## Development

```bash
# Build the project
cargo build

# Run in development (using the run script)
./run.sh focus

# Or run directly with cargo
PATH=/usr/bin:$PATH cargo run -- focus

# Run tests
cargo test
```

## Project Structure

- `src/main.rs` - Main CLI application with mood logic
- `Cargo.toml` - Dependencies and project configuration
- `PKGBUILD` - Arch Linux package build script
- `.SRCINFO` - AUR package metadata
- `install.sh` - Local installation script
- `run.sh` - Development run script
- `.cursorrules` - Development guidelines and preferences
