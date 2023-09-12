fn print_type_of<T>(log: &str, _: &T) {
    println!("##{}##:{}", log, std::any::type_name::<T>())
}
#[test]
fn vec_iter_test1() {
    let list = vec![1, 2, 3, 4, 5, 6];
    for i in &list {
        println! {"for:{}",i};
    }
    let list2: Vec<i32> = list.iter().map(|i| i * 3).collect();
    let list3: Vec<&i32> = list.iter().filter(|i| *i % 2 == 0).collect();

    print_type_of("list2", &list2);
    println!("{:?}", list2);
    println!("{:?}", list3);
}
