// maintain count / state
// maintain last_charge

// read file

// fn compare

use std::fs; 

fn main() {  
    let filename = "input3.txt"; 

    // vector
    let contents = open_file(&filename); 
    
    let bin_length = contents.get(0).unwrap().len(); 

    // most common
    let mut gamma:String = "".to_string(); 

    // least common
    let mut epsilon:String = "".to_string(); 

    // Double for loop is not advised
    for col_index in 0..bin_length {

        let mut count_zero = 0; 
        let mut count_one = 0; 

        for j in &contents { 
            let c: char = j.chars().nth(col_index).unwrap(); 
            let digit: u32 = c.to_digit(10).unwrap();  
            
            if digit == 1 {
                count_one = count_one + 1; 
            } else {
                count_zero = count_zero + 1; 
            }
        } 

        // determine which column is most common or least column
        if count_zero > count_one {
            gamma.push_str("0"); 
            epsilon.push_str("1");
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        } 
    }

    println!("gamma: {}", gamma); 
    println!("epi: {}", epsilon); 

    // Convert from binary string to int
    let gamma_value = u32::from_str_radix(&gamma, 2); 
    let epsilon_value = u32::from_str_radix(&epsilon, 2); 
    let result = gamma_value.as_ref().unwrap() * epsilon_value.as_ref().unwrap(); 
    // NEED to figure out how to multiply two results together

    println!("gamma_value: {}", gamma_value.unwrap()); 
    println!("epi_value: {}", epsilon_value.unwrap()); 
    println!("result: {}", result); 

}

fn open_file(file_name: &str) -> Vec<String> {
    let contents:Vec<String> = fs::read_to_string(file_name)
        .expect("File Error")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect(); 

    // output to a vector 
    return contents; 
}
