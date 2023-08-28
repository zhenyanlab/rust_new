struct  P{
    id :i32,
    next :Option<Box<P>>,
}

fn print_type_of<T>(_: &T) {

        println!("TYPE-PRINT:{}", std::any::type_name::<T>())
    
    }


//* 
//* 解引用
//as_ref  as_mut 去掉变量得&
//Box::new(p)把P挪入堆...
//Option(Some和None)不是包含关系非嵌套,是枚举Option的两种类型()
//判断None不能用!=和== 而是用方法..is_none
 //*/
fn print_link(p:P){
    print_type_of(&p);
    println!("{}",p.id);
    let mut pp = p;
    while !pp.next.is_none() {
        let vv = pp.next;
        print_type_of(&vv);
        pp = *(vv.unwrap());
        print_type_of(&pp);
        println!("{}",pp.id);
    }

}
#[test]
fn closefn(){
    let a: Option<Box<i32>> = Some(Box::new(5));
    let mut b = a.unwrap();
    *b = 8;
    print_type_of(&b);
    println!("{}",b);
    println!("{}",*b);
    println!("{}",&b);
    let mut rootP = P{id:0,next:None};
    let mut p1 = P{id:1,next:None};
    let p2 = P{id:2,next:None};
    print_type_of(&rootP);
    let mut bp1: Box<P> = Box::new(p1);
    bp1.next=Some(Box::new(p2));
    rootP.next=Some(bp1);
    //error
    // print_link(*bp1);
    print_link(rootP);
    // println!("{}",rootP);
}