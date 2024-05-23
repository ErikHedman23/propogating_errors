use std::fs;
use std::fs::read_to_string;
use std::io;

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = match fs::read_to_string(f1) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    s1.push('\n');
    s1.push_str(&s2);
    return Ok(s1);
}

fn main() {
    let result = read_and_combine("planets.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is... \n{}", s),
        Err(e) => println!("There was an error: {}", e),
    };
}

/*
Instead of doing the above code, what you can do instead to return an Err enumeration for the Result enum is to use a ?
at the end of the Result enum will perform the same task without the need of the match statement.

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?;
    let s2 = fs::read_to_string(f2)?; --- notice the ? after both of these enums?  This is to return an Err() if there is one
    s1.push('\n');
    s1.push_str(&s2);
    return Ok(s1);
}

*/
