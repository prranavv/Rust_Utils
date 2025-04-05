use std::fs;

use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(about="Move SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")]
struct Cli{
    source: Option<String>,
    destination: Option<String>
}

fn main()->Result<()>{
    let cli = Cli::parse();
    let source = cli.source.expect("expected a valid source");
    let destination = cli.destination.expect("expected a valid destinatoin");

    fs::rename(source, destination)?;
    Ok(())
}