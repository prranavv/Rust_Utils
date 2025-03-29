use std::{fs::File, io::{stdin, stdout, Read, Write}};
use std::path::Path;
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
            loop{
                let mut s = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut s).expect("enter a correct string");
            }
        }
    };
    let file_path = Path::new(&file);
    let file_path_extention_result = file_path.extension();
    let _= match file_path_extention_result{
        Some(val)=>{
            if val.to_str().unwrap()=="txt"{
                val
            }else{
                println!("File: {}",file);
                std::process::exit(-1);
            }
        }
        None=>{
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
