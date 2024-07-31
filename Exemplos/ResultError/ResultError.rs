


/////////////////////
/// 
/// use core::fmt;
use std::{path::Display, str::FromStr};

use std::fs::File;
use std::io::{Result, ErrorKind};



const path: &str = "poema.txt";

fn main() -> Result<()> {
    let file = File::open(path);
    match  file {
        Ok(ref file) => println!("File opened successfully: {}", path),
        Err(ref error) =>match error.kind() {
            ErrorKind::NotFound => {
                println!("Not Found!... creating file");
                handleCreateFile();
                println!("File opened successfully: {}", path );
            }
            _ => panic!("Problem opening the file: {:?}", error),
        }
    }
    let file = File::open(path).expect("Fatal error opening the file");

    Ok(())
}


fn handleCreateFile() -> () {
    let file = File::create(path).unwrap();
    
}


