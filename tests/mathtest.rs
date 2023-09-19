enum Coin {
    No1,
    No2,
    No3,
    No4,
}
// impl Copy for Coin {

//     fn Copy(){
//         print!("copy ");
//     }

// }

fn test_retrun(x: &Coin) -> i32 {
    match x {
        Coin::No1 => {
            println!("No1");
            1; //注意，这里没有分号，有分号是语句会报错，类型不匹配，没有分号则是表达式
            1
        }
        Coin::No2 => 2,
        Coin::No3 => 3,
        Coin::No4 => 4,
    }
}

#[test]
fn testMathreturn() {
    let x: Coin = Coin::No1;
    println!("{}", test_retrun(&x) == 1);
    assert_eq!(test_retrun(&x), 1);
    println!("return v = {}", test_retrun(&x));
}
