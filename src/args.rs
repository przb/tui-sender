use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Args {
    /// Directory containing message to pick from
    #[arg(short, long)]
    pub messages_dir: Option<PathBuf>,
}
