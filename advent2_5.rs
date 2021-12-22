// maintain count / state
// maintain last_charge

// read file

// fn compare

use std::fs; 

fn main() {  
   
    let mut horizontal = 0; 
    let mut depth = 0; 
    let mut aim = 0; 
    
    let filename = "test2.txt"; 

    // vector
    let contents = open_file(filename); 
    

    for content in contents {
        // Within loop closure to reset

        let result = return_coord(&content); 
        println!("direction: {0}, mag: {1}", result.0, result.1);
        
        let direction = result.0; 
        let magnitude = result.1;  

        match direction {
            "down" => {
                // + depth
                aim = aim + magnitude; 
            }, 
            "forward" => {
                horizontal = horizontal + magnitude; 
                depth = depth + aim * magnitude; 
            }, 
            "up" => {
                // - depth
                aim = aim - magnitude; 
            }, 
            _ => println!("Error"), 
        }
    }

    println!("Horizontal: {}", horizontal); 
    println!("Depth: {}", depth); 
    println!("Aim: {}", aim);
    println!("Answer: {}", horizontal*depth); 
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


// returns tuple of direction and units
fn return_coord(directions: &str) -> (&str, i32) {
    let mut a = directions.split_whitespace(); 

    // Requires that pattern is always direction and magnitude
    let direction: &str = a.next().unwrap(); 
    let magnitude: i32 = a.next().unwrap().parse().unwrap(); 

    let tuple: (&str, i32) = (direction, magnitude); 

    return tuple; 
}

