use std::{fs, path::Path};

use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(about="Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")]
#[command(name="cp")]
struct Cli{
    source: Option<String>,
    destination: Option<String>
}

fn main()-> Result<()>{
    let cli = Cli::parse();
    let source =match cli.source{
        Some(val)=>val,
        None=>{
            std::process::exit(-1);
        },
    };
    
    let destination =match cli.destination{
        Some(val)=>val,
        None=>{
            std::process::exit(-1);
        },
    };

    let source_path = Path::new(&source);
    let destination_path = Path::new(&destination);

    fs::rename(source_path, destination_path)?;

    Ok(())
}


