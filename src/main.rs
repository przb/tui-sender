mod app;
mod args;
mod ui;

use clap::Parser;
use color_eyre::Result;

use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let args = args::Args::parse();

    let app_result = App::from_args(args).run(&mut terminal);
    ratatui::restore();

    app_result
}
