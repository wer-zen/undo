use clap::{Parser, Subcommand};
use core::num;
use std::fs;
use std::process::Command;
#[derive(Parser)]
#[command(name = "undo!")]
#[command(about = "Undo your last command")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello
    Hello {
        /// Name to greet
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Show version
    Version,
    LastCommand {
        #[arg(short, long)]
        number: Option<usize>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => match name {
            Some(n) => println!("Hello, {}!", n),
            None => println!("Hello, world!"),
        },
        Commands::Version => println!("rust-command v1.0.0"),
        Commands::LastCommand { number } => {
            let n = number.unwrap_or(1); // Default to 1 if no number provided
            println!("{:?}", get_commands_from_history(n));
        }
    }
}
fn get_last_command() -> Result<String, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;

    // Try Fish first since you're using it
    let fish_history = home.join(".local/share/fish/fish_history");
    if fish_history.exists() {
        let content = fs::read_to_string(fish_history)?;
        for line in content.lines().rev() {
            if line.starts_with("- cmd: ") {
                return Ok(line[7..].to_string());
            }
        }
    }

    Err("No command found".into())
}

fn get_commands_from_history(n: usize) -> Result<String, Box<dyn std::error::Error>> {
    // n=0 is most recent, n=1 is previous, etc.
    let max = n + 1;

    let output = Command::new("fish")
        .args(&["-c", &format!("history --max={}", max)])
        .output()?;

    let history = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = history.lines().collect();

    if lines.len() > n {
        Ok(lines[n].trim().to_string())
    } else {
        Err(format!(
            "Not enough history entries (requested: {}, available: {})",
            n + 1,
            lines.len()
        )
        .into())
    }
}

fn undo_move_cmd(n: usize) -> Result<String, Box<dyn std::error::Error>> {
    let max = n + 1;

    let output = Command::new("fish")
        .args(&["-c", &format!("history --max={}", max)])
        .output()?;

    let history = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = history.lines().collect();

    if lines.len() > n {
        Ok(lines[n].trim().to_string())
    } else {
        Err(format!(
            "Not enough history entries (requested: {}, available: {})",
            n + 1,
            lines.len()
        )
        .into())
    }
}
