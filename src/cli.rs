use clap::Parser;

#[derive(Parser)]
#[command(author = "glomdom", version = "0.0.0a", about = "Rust to Luau compiler.", long_about = None)]
pub struct Cli {
    pub file: String,
}
