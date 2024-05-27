use anyhow::Result;
use clap::Parser;

mod app;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    verbose: bool,
}

pub fn main() -> Result<()> {
    let _args = Args::parse();

    // create app and run it
    let mut app = app::App::new()?;
    app.run()
}
