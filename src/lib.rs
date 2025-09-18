use std::{env, fs, process};


pub fn xor_encrypt(data:&[u8],key:u8)->Vec<u8>{
    data.iter().map(|byte| byte ^ key).collect()
}
pub fn get_data_and_key_xor()-> (Vec<u8>, u8 , String) {

    println!("======================================");
    println!("     for xor /enter/path/file/text    ");
    println!("======================================");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error:::usage:{} </file/path>",args[0]);
        process::exit(1);
    }
    let file_path= args[1].clone();
    let data: Vec<u8> = fs::read(&file_path).expect("Failed to read file");
    let key: u8 = 0xAB;
    
    (data,key,file_path)
}