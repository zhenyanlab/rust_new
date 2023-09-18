use std::fmt;
use std::error::Error;
use crate::MyErr::MYERROR1;

pub  enum  MyErr{
    COMMERR(Box<dyn Error>),
    MYERROR1(String),
}
pub type MyResult<T> = Result<T,MyErr>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
struct A{
    id : i32,
    name : String,
}
#[derive(Debug)]
struct B{
    id : i32,
}
impl  From<B> for A{
  fn from(value: B) -> Self{
      A{id:value.id,name : "def".to_string()}
  }

}
impl  fmt::Display for A{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A-tran-string: {},{}\n", self.id,self.name)
    }
}

fn test_throw_error() -> MyResult<A>{
    let mut a: A = A { id: 1,name : "sam".to_string() };
    let z=1;

    if z==z {
        let c = z/(z-z);
        // return MYERROR1("asdfa".to_string());
    }
    return Ok(a);
}

#[test]
fn from_into_test1() {
    println!("start-from-into");
    let mut a: A = A { id: 1,name : "sam".to_string() };
    let a2: A = A { id: 2 ,name : "sam2".to_string()};
    let b: B = B { id: 2 };
    let bb: B = B { id: 3 };
    //&&&&&&&&&&&&&&&&
    // From<B> for A
    //&&&&&&&&&&&&&&&&
    let a3  = A::from(b);
    let aa : A = bb.into();
    let astr = a.to_string();
    println!("{}",astr);
    println!("{:?}",a);
    println!("{:?}",aa);
    println!("{}",a<a2);
    println!("{}",a==a);
    println!("{}",a==a2);
    let mut a4 = a.clone();
    println!("{}{}{}",a,a3,a4);
    a4.name="a4".to_string();
    a.name = "a".to_string();
    //clone 后 互相不影响.
    println!("{}{}{}",a,a3,a4);


}