use clap::Parser;

/// CHIP-8 Emulator CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// Path to the CHIP-8 program file
    #[arg(short, long)]
    pub input: String,
}

