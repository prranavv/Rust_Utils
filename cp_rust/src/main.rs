use fs_extra::{self, copy_items, dir};
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

    let options = dir::CopyOptions::new();

    let mut from_path = Vec::new();
    from_path.push(source);
    copy_items(&from_path, destination, &options)?;
    Ok(())
}


