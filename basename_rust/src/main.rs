use clap::Parser;

#[derive(Parser)]
#[command(name="basename_rust")]
#[command(about="Print NAME with any leading directory components removed",long_about=None)]
#[command(version="1.0")]
struct Cli{
    name: Vec<String>,
    /// support multiple arguments and treat each as a NAME
    #[arg(short='a')]
    #[arg(long="multiple")]
    multiple_inputs: bool
}

fn main(){
    let cli=Cli::parse();
    
    match cli.multiple_inputs{
        false=>{
            let path = &cli.name[0];
            let iter =path.split("/");
            let i =iter.last();
            match i{
                Some(val)=>println!("{}",val),
                _=>(),
            }
        },
        true=>{
            let path = cli.name;
            let i =path.iter();
            for val in i{
                let iter = val.split("/");
                let x = iter.last();
                match x{
                    Some(val)=>println!("{}",val),
                    _=>(),
                }
            }
        }

    }
    
}