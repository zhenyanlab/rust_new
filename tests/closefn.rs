#[derive(Debug)]
struct  P{
    id :i32,
    next :Option<Box<P>>,
}

fn print_type_of<T>(log : &str,_: &T) {
        println!("##{}##:{}", log,std::any::type_name::<T>())
}


//* 
//* 解引用
//as_ref  as_mut 去掉变量得&
//Box::new(p)把P挪入堆...
//Option(Some和None)不是包含关系非嵌套,是枚举Option的两种类型()
//判断None不能用!=和== 而是用方法..is_none
 //*/
fn print_link(p:P){
    print_type_of("print_link",&p);
    println!("{}",p.id);
    let mut pp = p;
    while !pp.next.is_none() {
        let vv = pp.next;
        print_type_of("print_link_VV",&vv);
        pp = *(vv.unwrap());
        print_type_of("print_link_pp",&pp);
        println!("{}",pp.id);
    }

}
#[test]
fn closefn(){
    let a: Option<Box<i32>> = Some(Box::new(5));
    let mut b = a.unwrap();
    *b = 8;
    print_type_of("close_fn",&b);
    println!("{}",b);
    println!("{}",*b);
    println!("{}",&b);

    let mut rootP = P{id:0,next:None};
    let mut p1 = P{id:1,next:None};
    let p2 = P{id:2,next:None};
    // 这里的等号是单等号,也是判断相等的意思.
    if let P{id:4,next:None}=p2{
        //pass
    }
    let thread = Some(3);
    // 这里的等号是单等号,也是判断相等的意思.
    //true
    if let Some(3)= thread{
        println!("if let math express{:?}",Some(3));
    }
    print_type_of("rootp",&rootP);

    let mut bp1: Box<P> = Box::new(p1);
    //rust内存对象的装盘,必须按照一定的次序进行,先组装部分内存对象,再组装整体内存对象,
    //部分内存对象装配到整体内存对象后,就不能再动了?
    bp1.next=Some(Box::new(p2));
    rootP.next=Some(bp1);


    //error
    // print_link(*bp1);
    print_link(rootP);
    // println!("{}",rootP);
}