#[test]
fn boxTest() {
    let i = 5;
    let bi = Box::new(i);
    //装箱和解引用
    assert_eq!(bi, Box::new(5));
    assert_eq!(*bi, 5);
    println!("{}", bi);
    //闭包和lamada表达式
    let fn1 = |i: u8| i + 2;
    let b = fn1(3);
    println!("boxTest:{}", b);
}

struct Node {
    val: u8,
    next: Option<Box<Node>>,
}

impl Node {
    fn push(&mut self, node: Node) {}
}

fn linktest() {
    let mut root = Node { val: 0, next: None };
    let mut one = Node { val: 1, next: None };
    let two = Node { val: 2, next: None };
    one.next = Some(Box::new(two));
    root.next = Some(Box::new(one));

    foreachLink(&root);
}

fn foreachLink(root: &Node) {
    let mut iRoot = root;
    println!("{}", iRoot.val);
    // while(iRoot.next != None){
    let mut iRoot = iRoot.next.as_ref();
    foreachLink(iRoot.unwrap());
    // }
}
