use std::{fs, path::Path};

use clap::Parser;

#[derive(Parser)]
#[command(version="1.0")]
struct Cli{
    /// Enter Dir name to be created
    dir_name: Option<String>,

    /// make parent directories as needed
    #[arg(short='p',long="parents")]
    parent: bool,

    /// print a verbose messaage for each printed directory
    #[arg(short='v',long="verbose")]
    verbose: bool
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
            match cli.verbose{
                false=>(),
                true=>{
                    println!("mkdir: created {}",dir_name);
                }
            }
            fs::create_dir(dir_name)
        },
        true=>{
            match cli.verbose{
                false=>(),
                true=>{
                    let p=Path::new(&dir_name);
                    let ancestors=p.ancestors();
                    for a in ancestors{
                        if a.is_dir(){
                            break;
                        }
                        println!("mkdir: created {}",a.display());
                    }
                }
            }
            
            fs::create_dir_all(dir_name)
        }
    };
    match r {
        Err(e)=>{
            println!("{}",e);
            std::process::exit(-1);
        },
        Ok(_)=>{}
    };
}
