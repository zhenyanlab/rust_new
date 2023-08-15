use std::{fs::File, f32::consts::E};


// enum Result<T, E> {
//     Ok(T),
//     Err(E),
//     }


mod tool { // 使用 include!
    include!("../src/tool/ttt.rs");
}

//use crate::tool;
#[test]
fn toolTest(){
    print!("tool-start");
    println!("{}",tool::plus());
}

#[test]
fn fileTest(){
    println!("fileTest-start");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(ff) => {
            println!("{:?}",ff);
        },
        Err(e) =>{ println!("Error:{}",e);
            match e.kind() {
               _ => println!("{}","a"),
                
            }
    },
    
        
    };

}