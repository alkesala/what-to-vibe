use clap::Parser;
use colored::*;
use rand::seq::SliceRandom;
use std::{thread, time::Duration, io::{self, Write}};
// Crossterm is still needed for the dependency, but we're using simpler methods now

#[derive(Parser)]
#[command(name = "vibe")]
#[command(about = "Get themed responses based on your current mood")]
#[command(version)]
struct Cli {
    /// Your current mood (optional - will be randomly selected if not provided)
    #[arg(value_name = "MOOD")]
    mood: Option<String>,

    /// Show ASCII art for the selected mood
    #[arg(long)]
    ascii: bool,

    /// Enable GODMODE: override all moods with elite hacker drama
    #[arg(long)]
    godmode: bool,

    /// Start a pomodoro timer (in minutes, default 25)
    #[arg(long, value_name = "MINUTES", num_args = 0..=1, default_missing_value = "25")]
    timer: Option<u32>,

    /// Block distracting websites (requires sudo)
    #[arg(long)]
    focus_mode: bool,

    /// Set up VS Code workspace for the vibe
    #[arg(long)]
    workspace: bool,

    /// Launch interactive vibe selector
    #[arg(long)]
    interactive: bool,

    /// [HONEYPOT] Secret mode (shh...)
    #[arg(long, hide = true)]
    secret_mode: bool,

    /// [HONEYPOT] Debug mode with verbose output
    #[arg(long, hide = true)]
    debug: bool,

    /// [HONEYPOT] API key for cloud sync
    #[arg(long, value_name = "KEY", hide = true)]
    api_key: Option<String>,
}

#[derive(Debug)]
struct VibeResponse {
    theme: &'static str,
    music: &'static str,
    stack: &'static str,
    motto: &'static str,
}

fn get_vibe_response(mood: &str) -> Option<VibeResponse> {
    match mood.to_lowercase().as_str() {
        "focus" => Some(VibeResponse {
            theme: "Deep Ocean Blue",
            music: "Lo-fi beats, classical piano, ambient nature sounds",
            stack: "Rust, TypeScript, PostgreSQL, Docker",
            motto: "Flow state is the goal state",
        }),
        "chaotic" => Some(VibeResponse {
            theme: "Neon Cyberpunk",
            music: "EDM, industrial rock, experimental electronic",
            stack: "Python, JavaScript, MongoDB, Redis, WebSockets",
            motto: "Embrace the chaos, create order",
        }),
        "sadboi" => Some(VibeResponse {
            theme: "Melancholic Purple",
            music: "Indie folk, post-rock, sad piano ballads",
            stack: "Go, React, SQLite, simple APIs",
            motto: "Code through the feels",
        }),
        "energetic" => Some(VibeResponse {
            theme: "Sunset Orange",
            music: "Rock, punk, high-energy electronic, workout beats",
            stack: "Node.js, React, Firebase, WebRTC",
            motto: "Code like you're running out of time",
        }),
        "chill" => Some(VibeResponse {
            theme: "Forest Green",
            music: "Jazz, acoustic, nature sounds, smooth R&B",
            stack: "Python, Flask, SQLite, simple HTML/CSS",
            motto: "Take it easy, build it right",
        }),
        "creative" => Some(VibeResponse {
            theme: "Rainbow Spectrum",
            music: "Alternative, indie, experimental, world music",
            stack: "JavaScript, Three.js, WebGL, creative coding",
            motto: "Art and code are one",
        }),
        "productive" => Some(VibeResponse {
            theme: "Corporate Blue",
            music: "Instrumental hip-hop, productivity playlists, white noise",
            stack: "Java, Spring Boot, MySQL, Kubernetes",
            motto: "Efficiency is the ultimate form of beauty",
        }),
        "nostalgic" => Some(VibeResponse {
            theme: "Retro Sepia",
            music: "80s synthwave, classic rock, vinyl crackle",
            stack: "C++, OpenGL, legacy systems, retro computing",
            motto: "The future is built on the past",
        }),
        "adventurous" => Some(VibeResponse {
            theme: "Aurora Borealis",
            music: "Epic orchestral, adventure soundtracks, tribal drums",
            stack: "Rust, WebAssembly, blockchain, edge computing",
            motto: "Explore the unknown, build the impossible",
        }),
        "zen" => Some(VibeResponse {
            theme: "Minimalist White",
            music: "Meditation, zen gardens, silence, minimal ambient",
            stack: "Haskell, functional programming, pure functions",
            motto: "Less is more, simplicity is complexity resolved",
        }),
        _ => None,
    }
}

fn print_vibe_response(response: &VibeResponse) {
    println!();
    println!("{}", "╭─────────────────────────────────────────╮".cyan());
    println!("{}", "│              VIBE REPORT               │".cyan());
    println!("{}", "╰─────────────────────────────────────────╯".cyan());
    println!();
    
    println!("🎨 {}: {}", "Theme".yellow(), response.theme.green());
    println!("🎵 {}: {}", "Music".yellow(), response.music.green());
    println!("⚡ {}: {}", "Stack".yellow(), response.stack.green());
    println!("💭 {}: {}", "Motto".yellow(), response.motto.green());
    println!();
}

fn get_available_moods() -> Vec<&'static str> {
    vec!["focus", "chaotic", "sadboi", "energetic", "chill", "creative", "productive", "nostalgic", "adventurous", "zen"]
}

fn get_ascii_art(mood: &str) -> Option<&'static str> {
    match mood.to_lowercase().as_str() {
        "focus" => Some(r#"
   (  )   (   )  )
    ) (   )  (  (
    ( )  (    ) )
    _____________
   <__Focus!!!__>
    -------------
        \
         \
            .--.
           |o_o |
           |:_/ |
          //   \ \
         (|     | )
        /'\_   _/`\
        \___)=(___/
"#),
        "chaotic" => Some(r#"
      .-"""-.
     / .===. \
     \/ 6 6 \/
     ( \___/ )
 ___ooo__V__ooo___
|  CHAOS MODE!!!  |
 -----------------
     \   ^__^
      \  (oo)\_______
         (__)\       )\/\
             ||----w |
             ||     ||
"#),
        "sadboi" => Some(r#"
      .-''''-.
     /        \
    |  .--.  |
    | (    ) |
     \ '--' /
      '-..-'
   Sadboi Vibes
   ~~~~~~~~~~~
    (︶︹︺)
"#),
        "energetic" => Some(r#"
    🔥 ENERGETIC 🔥
     ⚡ ⚡ ⚡ ⚡ ⚡
    /           \
   |  ROCK ON!  |
    \           /
     ⚡ ⚡ ⚡ ⚡ ⚡
    🔥 FIRE UP! 🔥
"#),
        "chill" => Some(r#"
    🌿 CHILL 🌿
     ~~~~~~~~~
    /         \
   |  Relax   |
   |  & Code  |
    \         /
     ~~~~~~~~~
    🌿 VIBES 🌿
"#),
        "creative" => Some(r#"
    🎨 CREATIVE 🎨
     🌈 🌈 🌈 🌈
    /           \
   |  Create   |
   |  & Build  |
    \           /
     🌈 🌈 🌈 🌈
    🎨 ARTIST 🎨
"#),
        "productive" => Some(r#"
    💼 PRODUCTIVE 💼
     ⚡ ⚡ ⚡ ⚡ ⚡
    /             \
   |  Get Stuff  |
   |   Done!     |
    \             /
     ⚡ ⚡ ⚡ ⚡ ⚡
    💼 EFFICIENT 💼
"#),
        "nostalgic" => Some(r#"
    📼 NOSTALGIC 📼
     🎵 🎵 🎵 🎵
    /             \
   |  Retro Vibes |
   |  & Memories  |
    \             /
     🎵 🎵 🎵 🎵
    📼 CLASSIC 📼
"#),
        "adventurous" => Some(r#"
    🌌 ADVENTUROUS 🌌
     ⭐ ⭐ ⭐ ⭐ ⭐
    /               \
   |  Explore New  |
   |   Horizons!   |
    \               /
     ⭐ ⭐ ⭐ ⭐ ⭐
    🌌 EXPLORER 🌌
"#),
        "zen" => Some(r#"
    🧘 ZEN 🧘
     ☯️ ☯️ ☯️ ☯️
    /         \
   |  Breathe |
   |  & Code  |
    \         /
     ☯️ ☯️ ☯️ ☯️
    🧘 PEACE 🧘
"#),
        _ => None,
    }
}

fn print_godmode() {
    let lines = [
        "\x1b[92mInitializing GODMODE...\x1b[0m",
        "\x1b[91mBypassing mood matrix...\x1b[0m",
        "\x1b[96mInjecting quantum vibes...\x1b[0m",
        "\x1b[93mEstablishing neural uplink...\x1b[0m",
        "\x1b[95mSpawning 1337 threads...\x1b[0m",
        "\x1b[92m\n╔══════════════════════════════════════════════╗\x1b[0m",
        "\x1b[92m║           🦾 ELITE HACKER VIBE 🦾           ║\x1b[0m",
        "\x1b[92m╚══════════════════════════════════════════════╝\x1b[0m",
        "\x1b[96m🎨 Theme: Matrix Green on Black\x1b[0m",
        "\x1b[92m🎵 Music: Glitchcore, synthwave, modem noise\x1b[0m",
        "\x1b[93m⚡ Stack: Rust, Assembly, Brainfuck, Quantum APIs\x1b[0m",
        "\x1b[95m💭 Motto: 'There is no spoon. Only root.'\x1b[0m",
        "\x1b[92m\n[ACCESS GRANTED] Welcome, root overlord.\x1b[0m",
        "\x1b[92m$ sudo rm -rf / --no-preserve-root\x1b[0m",
        "\x1b[91m(Just kidding. Or am I?)\x1b[0m",
    ];
    for line in lines.iter() {
        println!("{}", line);
        thread::sleep(Duration::from_millis(400));
    }
}

fn pomodoro_timer(minutes: u32, mood: &str) {
    let total_seconds = minutes * 60;
    let mut remaining = total_seconds;
    
    // Get the full vibe response for the mood
    if let Some(response) = get_vibe_response(mood) {
        println!("\n{}", "╭─────────────────────────────────────────╮".cyan());
        println!("{}", "│            POMODORO VIBE               │".cyan());
        println!("{}", "╰─────────────────────────────────────────╯".cyan());
        println!();
        
        // Show the vibe theme
        println!("🎨 {}: {}", "Theme".yellow(), response.theme.green());
        println!("🎵 {}: {}", "Music".yellow(), response.music.green());
        println!("⚡ {}: {}", "Stack".yellow(), response.stack.green());
        println!("💭 {}: {}", "Motto".yellow(), response.motto.green());
        println!();
        
        // Show ASCII art
        if let Some(ascii) = get_ascii_art(mood) {
            println!("{}", ascii.cyan());
            println!();
        }
    }
    
    // Timer message based on mood
    let (emoji, msg) = match mood.to_lowercase().as_str() {
        "focus" => ("🔵", "Stay focused!"),
        "chaotic" => ("⚡", "Embrace the chaos!"),
        "sadboi" => ("💜", "Code through the feels!"),
        "energetic" => ("🔥", "Keep the energy up!"),
        "chill" => ("🌿", "Stay chill and code on!"),
        "creative" => ("🎨", "Let your creativity flow!"),
        "productive" => ("💼", "Productivity mode: ON!"),
        "nostalgic" => ("📼", "Old school grind!"),
        "adventurous" => ("🌌", "Explore new code worlds!"),
        "zen" => ("🧘", "Breathe and code."),
        _ => ("⏳", "Time to vibe!"),
    };
    
    println!("{} Pomodoro Timer: {} minutes | {}", emoji, minutes, msg);
    println!("{}", "Press Ctrl+C to stop early".dimmed());
    println!();
    
    // Countdown timer
    while remaining > 0 {
        let mins = remaining / 60;
        let secs = remaining % 60;
        print!("\r{} {:02}:{:02} remaining... ", emoji, mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        remaining -= 1;
    }
    
    println!("\r{} 00:00 Time's up! Take a break!           ", emoji);
    println!();
    
    // Break message with mood-specific ASCII
    println!("{}", "╭─────────────────────────────────────────╮".green());
    println!("{}", "│              BREAK TIME!               │".green());
    println!("{}", "╰─────────────────────────────────────────╯".green());
    println!();
    
    println!("{}\n", match mood.to_lowercase().as_str() {
        "focus" => "  (  )   (   )  )\n   ) (   )  (  (\n   ( )  (    ) )\n   _____________\n  <__Break!!!__>\n   -------------\n       \\\n        \\\n           .--.\n          |o_o |\n          |:_/ |\n         //   \\ \\\n        (|     | )\n       /'\\_   _/`\\\n       \\___)=(___/",
        "chaotic" => "      .-\"\"\"-.\n     / .===. \\\n     \\/ 6 6 \\/\n     ( \\___/ )\n ___ooo__V__ooo___\n|  BREAK CHAOS!  |\n -----------------",
        "sadboi" => "      .-''''-.\n     /        \\\n    |  .--.  |\n    | (    ) |\n     \\ '--' /\n      '-..-'\n   Break time, friend\n   ~~~~~~~~~~~~~~\n    (︶︹︺)",
        "energetic" => "    🔥 BREAK 🔥\n     ⚡ ⚡ ⚡ ⚡ ⚡\n    /           \\\n   |  Recharge  |\n   |  & Return  |\n    \\           /\n     ⚡ ⚡ ⚡ ⚡ ⚡\n    🔥 STRONGER 🔥",
        "chill" => "    🌿 BREAK 🌿\n     ~~~~~~~~~\n    /         \\\n   |  Relax   |\n   |  & Reset  |\n    \\         /\n     ~~~~~~~~~\n    🌿 RENEWED 🌿",
        "creative" => "    🎨 BREAK 🎨\n     🌈 🌈 🌈 🌈\n    /           \\\n   |  Refresh  |\n   |  & Create  |\n    \\           /\n     🌈 🌈 🌈 🌈\n    🎨 INSPIRED 🎨",
        "productive" => "    💼 BREAK 💼\n     ⚡ ⚡ ⚡ ⚡ ⚡\n    /             \\\n   |  Rest &    |\n   |  Recharge  |\n    \\             /\n     ⚡ ⚡ ⚡ ⚡ ⚡\n    💼 READY 💼",
        "nostalgic" => "    📼 BREAK 📼\n     🎵 🎵 🎵 🎵\n    /             \\\n   |  Remember  |\n   |  & Reflect  |\n    \\             /\n     🎵 🎵 🎵 🎵\n    📼 WISER 📼",
        "adventurous" => "    🌌 BREAK 🌌\n     ⭐ ⭐ ⭐ ⭐ ⭐\n    /               \\\n   |  Rest &      |\n   |  Prepare     |\n    \\               /\n     ⭐ ⭐ ⭐ ⭐ ⭐\n    🌌 READY 🌌",
        "zen" => "    🧘 BREAK 🧘\n     ☯️ ☯️ ☯️ ☯️\n    /         \\\n   |  Breathe |\n   |  & Reset  |\n    \\         /\n     ☯️ ☯️ ☯️ ☯️\n    🧘 CENTERED 🧘",
        _ => "\n  (•_•)  ( •_•)>⌐■-■  (⌐■_■)\nBreak like a boss!",
    });
}

fn honeypot_response() {
    println!("\n{}", "🔐 SECRET MODE ACTIVATED 🔐".red().bold());
    println!("{}", "Accessing classified vibe database...".yellow());
    thread::sleep(Duration::from_millis(800));
    println!("{}", "Bypassing security protocols...".yellow());
    thread::sleep(Duration::from_millis(600));
    println!("{}", "Decrypting quantum vibe matrix...".yellow());
    thread::sleep(Duration::from_millis(700));
    println!();
    println!("{}", "╭─────────────────────────────────────────╮".red());
    println!("{}", "│           CLASSIFIED VIBES             │".red());
    println!("{}", "╰─────────────────────────────────────────╯".red());
    println!();
    println!("🎨 Theme: Invisible Spectrum");
    println!("🎵 Music: Government surveillance frequencies");
    println!("⚡ Stack: Quantum computers, classified APIs, black ops tools");
    println!("💭 Motto: 'The best vibes are the ones you can't see'");
    println!();
    println!("{}", "⚠️  WARNING: This mode is completely fake!".green());
    println!("{}", "   You've been honeypotted! 🕷️".green());
    println!();
}

fn debug_honeypot() {
    println!("\n{}", "🐛 DEBUG MODE ENABLED 🐛".cyan().bold());
    println!("{}", "Loading verbose output...".yellow());
    thread::sleep(Duration::from_millis(500));
    println!("{}", "Analyzing vibe quantum states...".yellow());
    thread::sleep(Duration::from_millis(400));
    println!("{}", "Calculating mood entropy...".yellow());
    thread::sleep(Duration::from_millis(300));
    println!();
    println!("{}", "DEBUG INFO:".cyan().bold());
    println!("  - Vibe entropy: 42.1337");
    println!("  - Mood quantum state: |ψ⟩ = α|happy⟩ + β|sad⟩");
    println!("  - Debug level: OVER 9000");
    println!("  - Reality check: This is all fake!");
    println!();
}

fn api_honeypot(api_key: &str) {
    println!("\n{}", "☁️  CLOUD SYNC INITIALIZED ☁️".blue().bold());
    println!("{}", "Connecting to vibe cloud...".yellow());
    thread::sleep(Duration::from_millis(600));
    println!("{}", "Authenticating with API key...".yellow());
    thread::sleep(Duration::from_millis(500));
    println!("{}", "Syncing your vibes to the cloud...".yellow());
    thread::sleep(Duration::from_millis(400));
    println!();
    println!("{}", "API Key: {}...{}".cyan(), &api_key[..8], "***");
    println!("{}", "Status: Connected to fake cloud service");
    println!("{}", "Sync: Your vibes are now stored in the void");
    println!("{}", "Security: This is completely made up!");
    println!();
}

fn interactive_mode() {
    let moods = get_available_moods();
    let mut selected_index = 0;
    
    // Simple interactive mode without alternate screen
    loop {
        // Clear screen using simple method
        print!("\x1B[2J\x1B[1;1H"); // ANSI clear screen
        io::stdout().flush().unwrap();
        
        // Print header
        println!("{}", "╭─────────────────────────────────────────────────────────╮".cyan());
        println!("{}", "│                🎵 VIBE SELECTOR 🎵                   │".cyan());
        println!("{}", "╰─────────────────────────────────────────────────────────╯".cyan());
        println!();
        println!("{}", "Use ↑↓ arrows to navigate, Enter to select, Ctrl+C to exit".yellow());
        println!();
        
        // Print mood list
        for (i, mood) in moods.iter().enumerate() {
            if i == selected_index {
                print!("{}", "  ▶ ".green());
                println!("{}", mood.to_uppercase().green().bold());
            } else {
                print!("    ");
                println!("{}", mood);
            }
        }
        
        println!();
        println!("{}", "╭─────────────────────────────────────────────────────────╮".blue());
        println!("{}", "│                     PREVIEW                           │".blue());
        println!("{}", "╰─────────────────────────────────────────────────────────╯".blue());
        println!();
        
        // Show preview of selected mood
        let selected_mood = moods[selected_index];
        if let Some(response) = get_vibe_response(selected_mood) {
            println!("🎨 {}: {}", "Theme".yellow(), response.theme.green());
            println!("🎵 {}: {}", "Music".yellow(), response.music.green());
            println!("⚡ {}: {}", "Stack".yellow(), response.stack.green());
            println!("💭 {}: {}", "Motto".yellow(), response.motto.green());
        }
        
        // Show ASCII art preview
        if let Some(ascii) = get_ascii_art(selected_mood) {
            println!();
            println!("{}", ascii.cyan());
        }
        
        // Simple input handling
        let mut input = String::new();
        print!("{}", "Enter choice (↑/↓/Enter/q): ".yellow());
        io::stdout().flush().unwrap();
        
        if io::stdin().read_line(&mut input).is_ok() {
            match input.trim() {
                "w" | "k" | "up" => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                "s" | "j" | "down" => {
                    if selected_index < moods.len() - 1 {
                        selected_index += 1;
                    }
                }
                "" | "enter" => {
                    // Print final selection
                    let selected_mood = moods[selected_index];
                    println!("\n{}", "🎉 Selected vibe:".green().bold());
                    println!("{}", selected_mood.to_uppercase().cyan().bold());
                    println!();
                    
                    // Print full vibe response
                    if let Some(response) = get_vibe_response(selected_mood) {
                        print_vibe_response(&response);
                    }
                    
                    // Print ASCII art
                    if let Some(ascii) = get_ascii_art(selected_mood) {
                        println!("{}", ascii.cyan());
                    }
                    
                    return;
                }
                "q" | "quit" | "exit" => {
                    println!("\n{}", "👋 Vibe selection cancelled".yellow());
                    return;
                }
                _ => {
                    // Try to parse as number
                    if let Ok(num) = input.trim().parse::<usize>() {
                        if num > 0 && num <= moods.len() {
                            selected_index = num - 1;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    
    // Honeypot responses (check these first)
    if cli.secret_mode {
        honeypot_response();
        return;
    }
    
    if cli.debug {
        debug_honeypot();
        return;
    }
    
    if let Some(api_key) = cli.api_key {
        api_honeypot(&api_key);
        return;
    }
    
    if cli.godmode {
        print_godmode();
        return;
    }

    // --interactive
    if cli.interactive {
        interactive_mode();
        return;
    }

    // --timer
    if let Some(timer_minutes) = cli.timer {
        let mood = cli.mood.clone().unwrap_or_else(|| {
            let moods = get_available_moods();
            moods.choose(&mut rand::thread_rng()).unwrap().to_string()
        });
        pomodoro_timer(timer_minutes, &mood);
        return;
    }

    // --focus-mode
    if cli.focus_mode {
        println!("[FOCUS MODE] Blocking distracting websites (feature coming soon)");
        // TODO: Implement focus mode
        return;
    }

    // --workspace
    if cli.workspace {
        println!("[WORKSPACE] Setting up VS Code for the vibe (feature coming soon)");
        // TODO: Implement workspace setup
        return;
    }
    
    let mood = match cli.mood {
        Some(m) => m,
        None => {
            let available_moods = get_available_moods();
            let random_mood = available_moods.choose(&mut rand::thread_rng())
                .expect("Failed to select random mood");
            println!("{}", "🎲 No mood specified, choosing randomly...".yellow());
            random_mood.to_string()
        }
    };
    
    if cli.ascii {
        if let Some(ascii) = get_ascii_art(&mood) {
            println!("{}", ascii.cyan());
        }
    }
    
    match get_vibe_response(&mood) {
        Some(response) => {
            print_vibe_response(&response);
        }
        None => {
            eprintln!("{}", "❌ Unknown mood!".red());
            eprintln!("Available moods: {}", "focus, chaotic, sadboi".cyan());
            std::process::exit(1);
        }
    }
} 