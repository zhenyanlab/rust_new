use serde::Deserialize;
use serde::Serialize;
use serde_json::json;


//添加依赖一定要加--features derive 要不然会一直报错... 找不到包...
//cargo add serde --features derive
//cargo add serde_json serde_yaml

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct User {
  name: String,
  age: u8,
  blog: String,
  addr: String,
}

#[tokio::test]
async fn serde_test() {
  println!("test");

  let p  = User{
    name : String::from("wohaha"),
    age : 34,
    blog: String::from("http://rustcn.cn"),
    addr : String::from("bj"),
  };

   let serialized = serde_json::to_string(&p).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("json-str = {}", serialized);

    // Convert the JSON string back to a Point.
    let p2: User = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("obj = {:?}", p2);
    let v= vec![1,2,3];
    println!("{:?}",v);

}
