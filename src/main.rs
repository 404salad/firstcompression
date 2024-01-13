use std::fs;
/* TODO
 * write a uncompression algo 
 * compare and check if correct 
 * parallelize it
 */

fn naive_compress(input_data: String)->String{
    let mut encoded_string = String::new();
    let mut n = 0;
    while n<input_data.len()-1{
        encoded_string += &(input_data.chars().nth(n).unwrap()).to_string();
        let mut count = 0;
        let start_char = input_data.chars().nth(n).unwrap();
        let mut walking_char = input_data.chars().nth(n+count).unwrap();
        while start_char==walking_char{
            count+=1;
            walking_char = input_data.chars().nth(n+count).unwrap();
        }
        if count>1 {
            encoded_string += &count.to_string();
        }
        n+=count;
    }
    encoded_string
}

fn naive_uncompress(input_data: String) -> String {
    let mut uncompressed_string = String::new();
    let mut i = 0;
    while i < input_data.len()-1 {
        let current_char = input_data.chars().nth(i).unwrap();
        let possible_count = input_data.chars().nth(i+1).unwrap();
        if possible_count.is_digit(10) {
            for _ in 0..possible_count.to_digit(10).unwrap(){
                uncompressed_string+=&current_char.to_string();
            }
        }
        i+=1;
    }
    uncompressed_string
}
fn main() -> Result<(), std::io::Error>{
    let file_path = "input.txt";
    let input_data = fs::read_to_string(file_path)?;
    println!("{}",input_data);
    println!("{}",naive_compress(input_data));
    Ok(())
}
