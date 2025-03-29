use clap::Parser;

#[derive(Parser)]
#[command(name="basename_rust")]
#[command(about="Print NAME with any leading directory components removed",long_about=None)]
#[command(version="1.0")]
struct Cli{
    name: String
}

fn main(){
    let cli=Cli::parse();

    let path = cli.name;
    let iter =path.split("/");
    let i =iter.last();
    match i{
        Some(val)=>println!("{}",val),
        _=>(),
    }
}