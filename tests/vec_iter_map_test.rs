fn print_type_of<T>(log: &str, _: &T) {
    println!("##{}##:{}", log, std::any::type_name::<T>())
}

#[test]
fn vec_iter_test1() {
    let list = vec![1, 2, 3, 4, 5, 6];
    let list4 = vec![7, 8, 9, 10, 11, 12];
    for i in &list {
        println! {"for:{}", i};
    }
    let list2: Vec<i32> = list.iter().map(|i| i * 3).collect();
    let list3: Vec<&i32> = list.iter().filter(|i| *i % 2 == 0).collect();
    let list5: Vec<_> = list.iter().chain(list4.iter()).map(|x| x * 2).collect();
    let sum = list.iter().fold(0, |a, b| a + b);
    println!("{}", sum);
    print_type_of("list2", &list2);
    println!("{:?}", list2);
    println!("{:?}", list3);
    println!("{:?}", list5);
    let inta = "|56";
    let refa = &inta;

    let refa2 = &refa;
    print_type_of("refa", &refa);
    print_type_of("refa2", &refa2);
    println!("{}", refa2);
    println!("{}", *refa2);
    let strb = refa2.to_string() + "asdfasdf";
    strb.rsplit("a");
    println!("{}", strb);
}
