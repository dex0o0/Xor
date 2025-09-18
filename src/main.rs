use std::fs;
use xor::{xor_encrypt,get_data_and_key_xor};

fn main() {
    let (data ,key, file_path): (Vec<u8>, u8,String) = get_data_and_key_xor();
    let xor = xor_encrypt(&data,key);
    let out_path = format!("{}.enc",file_path);
    fs::write(&out_path, xor).expect("failed to write file");
    println!("complet: {}",out_path);
}
