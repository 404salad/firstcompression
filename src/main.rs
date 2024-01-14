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

fn naive_uncompress(input_data: String) -> String {
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
    println!("{}",input_data);
    println!("{}",naive_compress(input_data.clone()));
    println!("");
    println!("{}",naive_uncompress(naive_compress(input_data)));
    Ok(())
}
