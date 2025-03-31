use clap::Parser;
use data_encoding::BASE32;
use std::path::Path;
use std::fs;

#[derive(Parser)]
#[command(
name="base32_rust",
version="1.0",
about="encode/decode data and print to standard output With no FILE, or when FILE is -, read standard input.")]
struct Cli{
    file_path: Option<String>,

    /// decode data (hasn't been implementeted yet)
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
    let input: &[u8]=f.as_bytes();
    match cli.decode{
        false=>{
            let encoded_data=BASE32.encode(input);
            println!("{}",encoded_data); 
        },
        true=>{
            // let mut output = vec![0; BASE32.decode_len(input.len()).unwrap()];
            // let len = BASE32.decode_mut(input, &mut output).unwrap();
            // let r = &output[0..len];
            // let result=String::from_utf8(r.to_vec()).unwrap();
            // println!("{}",result);
        }   
    }
    
}
