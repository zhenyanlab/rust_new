use std::{f32::consts::E, fs::File};

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
//     }

mod tool {
    // 使用 include!
    include!("../src/tool/ttt.rs");
}

//use crate::tool;
#[test]
fn toolTest() {
    print!("tool-start");
    println!("{}", tool::plus());
}

#[test]
fn fileTest() {
    println!("fileTest-start");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(ff) => {
            println!("{:?}", ff);
        }
        Err(e) => {
            println!("Error:{}", e);
            match e.kind() {
                _ => println!("{}", "a"),
            }
        }
    };
}

#[test]
fn appstoreTest() {
    println!(
        "{}",
        match tool::find_store("windows") {
            Some(s) => s,
            None => "Not a valid mobile OS",
        }
    );

    println!(
        "{}",
        match tool::find_store("IOS") {
            Some(s) => s,
            None => "Not a valid mobile OS",
        }
    );
    println!(
        "{}",
        match tool::find_store("android") {
            Some(s) => s,
            None => "Not a valid mobile OS",
        }
    );
    assert_eq!(tool::find_store("IOS"), Some("app store"));
    assert_eq!(tool::find_store("windows"), None);
    if tool::find_store("IOS") == Some("app store") {
        println!("ios app store is match");
    }
    assert_ne!(tool::find_store("windows"), Some("app store"));
}
