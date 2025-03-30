use clap::Parser;
use base32::{encode,Alphabet};

#[derive(Parser)]
#[command(name="base32_rust",version="1.0",
about="encode/decode data and print to standard output With no FILE, or when FILE is -, read standard input.")]
struct Cli{
    file_path: Option<String>
}

fn main(){
    let cli=Cli::parse();
    let file_path_result = cli.file_path;
    let file_path = match file_path_result{
        Some(f)=>f,
        None=>{
            eprintln!("not a valid path name");
            std::process::exit(-1);
        }
    };
    let bytes_file_path = file_path.as_bytes();
    let a =Alphabet::Rfc4648 { padding: false };
    let encoded_bytes = encode(a, bytes_file_path);
    println!("{}",encoded_bytes);
}