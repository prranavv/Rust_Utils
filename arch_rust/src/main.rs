use std::env;

fn main(){
    let arch = env::consts::ARCH;
    println!("{}",arch);        
}