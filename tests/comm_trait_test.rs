use crate::MyErr::MYERROR1;
use rand::Rng;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MyErr {
    COMMERR(Box<dyn Error>),
    MYERROR1(String),
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: \n")
    }
}

pub type MyResult<T> = Result<T, MyErr>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct A {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct B {
    id: i32,
}
impl A{
    fn new () ->Self{
        return A{id:-1,name:"is def name".to_string()};
    }
}
impl From<B> for A {
    fn from(value: B) -> Self {
        A {
            id: value.id,
            name: "def".to_string(),
        }
    }
}

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A-tran-string: {},{}\n", self.id, self.name)
    }
}

fn test_throw_error() -> Result<Option<A>, MyErr> {
    let mut a: A = A {
        id: 1,
        name: "test_throw_error-str".to_string(),
    };
    let z = 1;
    let mut rng = rand::thread_rng();
    let ii = rng.gen_range(1..=10);
    println!("rand num Integer: {}", ii);

    if ii < 9 {
        //let c = z/(z-z);
        return Err(MYERROR1("asdfa".to_string()));
    }
    return Ok(Some(a));
}

fn callFn() -> Result<Option<A>, MyErr> {
    println!("{}", "start-fun-callFn");
    let a = test_throw_error();
    if let MyErr=a {
        println!("error is catch");
    }
    //如果发生异常,此句下面的代码可能执行不到,并直接返回Err()这个枚举类
    let p = test_throw_error()?;

    // let p = test_throw_error().unwrap_or_else(|_|{A{id:111,name:"ERROR".to_string()}});
    print_type_of("没有发生异常就会执行到这儿.##p##", &p);
    // println!("{}", p.unwrap());
    Ok(p)
}

#[test]
fn from_into_test1() {
    println!("start-from-into");
    let mut a: A = A {
        id: 1,
        name: "sam".to_string(),
    };
    let a2: A = A {
        id: 2,
        name: "sam2".to_string(),
    };
    let b: B = B { id: 2 };
    let bb: B = B { id: 3 };
    //&&&&&&&&&&&&&&&&
    // From<B> for A
    //&&&&&&&&&&&&&&&&
    let a3 = A::from(b);
    let aa: A = bb.into();
    let astr = a.to_string();
    println!("{}", astr);
    println!("{:?}", a);
    println!("{:?}", aa);
    println!("{}", a < a2);
    println!("{}", a == a);
    println!("{}", a == a2);
    let mut a4 = a.clone();
    println!("{}{}{}", a, a3, a4);
    a4.name = "a4".to_string();
    a.name = "a".to_string();
    //clone 后 互相不影响.
    println!("{}{}{}", a, a3, a4);
    let pp = callFn().unwrap_or_else(|_|{Some(A::new())});
    print_type_of("pp", &pp);
    if let None = pp{
            println!("pp is none")
        }

    let pp = callFn().unwrap_or_else(|_|{None});
    print_type_of("pp", &pp);
    if let None = pp{
        println!("pp is none")
    }

    // println!("{}",pp);
}

fn print_type_of<T>(log: &str, _: &T) {
    println!("##{}##:{}", log, std::any::type_name::<T>())
}
