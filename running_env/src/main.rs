fn main() {
    println!("Hello, world!");
    let result: Result<u8, String> = calc(56,200);
    error_handling(result); 
}

fn calc(x:u8, y:u8) -> Result<u8, String>{
    match x.checked_add(y) {
        Some(sum) => Ok(sum as u8),
        None => Err(String::from("An error occurred during calculation"))
    }
}

fn error_handling(result:Result<u8, String>){
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
