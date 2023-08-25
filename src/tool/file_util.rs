use std::fs::File;
use std::io::{ErrorKind, Read};

use std::env;
use std::fs;

use actix_web::Error;
//fn write_string_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
fn write_string_to_file(path: &str, data: &str) -> Result<(), Error> {
    fs::write(path, data)?;
    Ok(())
}

pub  fn file_test()->std::io::Result<()>{
    let file_name = "hello.txt";
    fs::remove_file(file_name)?;
    let f: File = File::open(file_name).unwrap_or_else(|error| {
        println!("file err");
        if error.kind() == ErrorKind::NotFound {
            println!("file err2");
            let fff = File::create("hello.txt").unwrap_or_else(|error| {
                println!("file err3");
                panic!("@@Problem creating the file: {:?}", error);
            });
            let res = write_string_to_file(file_name,"@241234127123986412hfsadhfalh");
            res.unwrap_or_else(|error|{
                println!("file err4");
            });
            return fff ;
        } else {
            println!("file err5");
            panic!("@@Problem opening the file: {:?}", error);
        }
    });
    let mut buf: Result<String, std::io::Error> = fs::read_to_string(file_name);
            
    //let mut buf = String::new();
    //f.read_to_string(  buf);
    let buf: String = buf.unwrap_or_else(|error| {
        println!("?????????????{}",error);
        return String::new();
    });
    println!("########################|{}|",buf);
    Ok(())
}