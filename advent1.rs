// maintain count / state
// maintain last_charge

// read file

// fn compare

use std::fs; 

fn main() {
    let mut count:i32 = 0;  

    let filename = "input.txt"; 

    // vector
    let contents = open_file(filename); 

    // set last value to first vector value
    let mut last_value:i32 = contents[0];

    for item in contents {
        if compare(last_value, item) {
            count = count + 1; 
        }

        last_value = item;
    }

    println!("Count: {}", count); 
}

fn compare(last_depth: i32, current_depth: i32) -> bool {
    if current_depth > last_depth {
        return true; 
    }

    false
}

fn open_file(file_name: &str) -> Vec<i32> {
    let contents:Vec<i32> = fs::read_to_string(file_name)
        .expect("File Error")    
        .lines()
        .map(|x| x.parse().unwrap())
        .collect(); 

    return contents; 
}