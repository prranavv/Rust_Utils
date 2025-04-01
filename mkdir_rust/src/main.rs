use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(version="1.0")]
struct Cli{
    /// Enter Dir name to be created
    dir_name: Option<String>,

    /// make parent directories as needed
    #[arg(short='p',long="parents")]
    parent: bool
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
    let r =match cli.parent{
        false=>{
            fs::create_dir(dir_name)
        },
        true=>{
            fs::create_dir_all(dir_name)
        }
    };
    match r {
        Err(e)=>println!("{}",e),
        _=>()
    }
    

}
