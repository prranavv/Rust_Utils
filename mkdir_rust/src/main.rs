use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version="1.0")]
struct Cli{
    /// Enter Dir name to be created
    dir_name: Option<String>,
}

fn main(){
    let cli=Cli::parse();
    let dir_name_opt = cli.dir_name;
    let dir_name = match dir_name_opt{
        Some(val)=>{
            val
        },
        None=>{
            println!("no name specified");
            std::process::exit(-1);
        }
    };
    let r = fs::create_dir(dir_name);
    match r{
        Err(e)=>{
            println!("{}",e)
        }
        _=>()
    }
}
