use std::fs;
use rayon::prelude::*;

fn naive_compress(input_data: &str)->String{
    let mut encoded_string = String::new();
    let mut n = 0;
    while n<input_data.len()-1{
        encoded_string.push_str(&(input_data.chars().nth(n).unwrap()).to_string());
        let mut count = 0;
        let start_char = input_data.chars().nth(n).unwrap();
        let mut walking_char = input_data.chars().nth(n+count).unwrap();
        while start_char==walking_char{
            count+=1;
            walking_char = input_data.chars().nth(n+count).unwrap();
        }
        if count>1 {
            encoded_string.push_str(&count.to_string());
        }
        n+=count;
    }
    encoded_string
}

fn naive_uncompress(input_data: &str) -> String {
    let mut uncompressed_string = String::new();
    let mut countarray:Vec<u32> = vec![];
    let mut number_str = String::new();
    for letter in input_data.chars() {
        if letter.is_numeric(){
           number_str += &letter.to_string(); 
        }
        else {
            let number:u32 = match number_str.parse(){
                Ok(num) => num,
                Err(_) =>{
                    1 
                }
            };
            countarray.push(number);
            number_str = String::new();
        }
    }
    // for handling edge case
    let number:u32 = match number_str.parse(){
        Ok(num) => num,
        Err(_) =>{
            1 
        }
    };
    countarray.push(number);

    let mut i =1;
    for letter in input_data.chars(){
        if letter.is_numeric()  {
            continue;
        }
        for _ in 0..countarray[i] {
            uncompressed_string.push_str(&letter.to_string());
        }
        i+=1;
    }

    uncompressed_string
}



fn main() -> Result<(), std::io::Error>{
 let file_path = "input.txt";
 let input_data = fs::read_to_string(file_path)?;
 let compressed_data = naive_compress(&input_data);

 println!("{}",input_data);
 println!("{}",compressed_data);
 println!("");

 // Split the compressed data into chunks
let chunks: Vec<String> = compressed_data.as_str().chars().collect::<Vec<_>>().chunks(1000).map(|chunk| chunk.into_iter().collect::<String>()).collect();

 let uncompressed_chunks: Vec<String> = chunks.par_iter().map(|chunk| naive_uncompress(chunk)).collect();

 let uncompressed_data: String = uncompressed_chunks.join("");

 println!("{}",uncompressed_data);
 Ok(())
}

