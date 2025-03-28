use std::{fs::File, io::Read};

use clap::Parser;

#[derive(Parser)]
#[command(name="cat_rust")]
#[command(version="1.0")]
#[command(about="concatenate files and print on the standard output",long_about= None)]
struct Cli{
    /// This is the name of the file that you want to concatenate on the stdout
    file_name: Option<String>
}

fn main(){
    let cli = Cli::parse();
    let file_name = cli.file_name;
    let file = match file_name{
        Some(val)=>val,
        None=>{
            println!("No filename given");
            std::process::exit(-1);
        }
    };
    let file_result = File::open(file);
    let mut contents = String::new();
    let mut f = match file_result{
        Ok(fil)=>fil,
        Err(e)=>{
            println!("{}",e);
            std::process::exit(-1);
        }
    };
    let res =f.read_to_string(&mut contents);
    match res{
        Err(e)=>{
            println!("{}",e);
            std::process::exit(-1);
        },
        _=>()
    };
    println!("{}",contents);
}
