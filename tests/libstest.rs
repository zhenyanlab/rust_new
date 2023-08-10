use actix_web::error;

// extern  crate rust_new;
//use rust_new;
//error
// mod rust_new;
#[test]
fn testcase1(){
    println!("testcase1");
}

#[test]
fn testcase2(){
    println!("testcase2");
    // panic!("ERROR");
}


#[test]
fn testcase3(){
    println!("testcase3");
    let c=rust_new::wowo2(2,3);
    println!("{}",rust_new::wowo2(2,3));
    println!("{}",rust_new::com::wowo(2,3));

}
