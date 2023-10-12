use std::str;



//  strs = str::from_utf8_unchecked(&sparkle_heart);
//  |^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
#[test]
fn unsafe_test1() {
    println!("start");
    let sparkle_heart = vec![240, 159, 146, 150];
    let mut strs  = "";
    unsafe {
        strs = str::from_utf8_unchecked(&sparkle_heart);
    }
    println!("{}",&strs);
    assert_eq!("💖",strs);
}