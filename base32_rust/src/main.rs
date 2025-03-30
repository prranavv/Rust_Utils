use clap::Parser;
use base32::{decode, encode, Alphabet};
use std::path::Path;
use std::fs;
#[derive(Parser)]
#[command(name="base32_rust",version="1.0",
about="encode/decode data and print to standard output With no FILE, or when FILE is -, read standard input.")]
struct Cli{
    file_path: Option<String>,

    /// decode data
    #[arg(short='d',long="decode")]
    decode: bool
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
    let path=Path::new(&file_path);
    let f = fs::read_to_string(path).unwrap();
    match cli.decode{
        false=>{
            let a =Alphabet::Rfc4648 { padding: false };
            let bytes_file_path = f.as_bytes();
            let encoded_bytes = encode(a, bytes_file_path);
            println!("{}",encoded_bytes);
            
        },
        true=>{
            let a = Alphabet::Rfc4648 { padding: false };
            let decoded_bytes_op = decode(a, &file_path);
            let decoded_bytes=match decoded_bytes_op{
                Some(val)=>val,
                None=>{
                    std::process::exit(-1);
                }
            };
            let result=String::from_utf8(decoded_bytes).unwrap();
            println!("{}",result);
        }
    }
    
}