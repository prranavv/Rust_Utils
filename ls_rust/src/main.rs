use std::{env, fs, path::Path};

use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
struct Cli{
    path: Option<String>
}

fn display_contents(path: Option<String>){
    match path{
        Some(path)=>{
            let path = Path::new(&path);
            let files =fs::read_dir(path).unwrap();
            for file in files{
                if let Ok(file)=file{
                    print!("{} ",file.file_name().into_string().unwrap())
                }
            }
        },
        None=>{
            let path=env::current_dir().unwrap();
            let files=fs::read_dir(path).unwrap();
            for file in files{
                if let Ok(file)=file{
                    print!("{} ",file.file_name().into_string().unwrap())
                }
            }
        }
    }
}

fn main()->Result<()>{
    let cli=Cli::parse();
    let path=cli.path;
    display_contents(path);
    Ok(())
}