use std::{fs, io::{stdin, stdout, Write}};

use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(about="Move SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")]
struct Cli{
    source: Option<String>,
    destination: Option<String>, 

    /// prompt before override
    #[arg(short='i',long="interactive")]
    interactive: bool
}

fn main()->Result<()>{
    let cli = Cli::parse();
    let source = cli.source.expect("expected a valid source");
    let destination = cli.destination.expect("expected a valid destinatoin");

    match cli.interactive{
        true=>{
            let mut s =String::new();
            print!("mv-rust: overwrite '{}'",destination);
            let _ = stdout().flush();
            stdin().read_line(&mut s)?;
            let s =s.trim();
            match s{
                "y" |"yes" =>{

                }

                _=> std::process::exit(-1)
            }
        }
        false=>std::process::exit(-1)
    }
    fs::rename(source, destination)?;
    Ok(())
}