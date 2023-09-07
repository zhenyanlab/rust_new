use mysql as my;
use mysql::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[warn(dead_code)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct  P {
    id : i32,
    name : String,
    age : i32,
    address : String,
}

impl P {
    pub fn new() -> P {
        P {
            id:0,
            name:String::from(""),
            age:0,
            address : String::from("")
        }
    }
}

#[test]
fn mysql_Test_conn(){
    println!("mysql_Test_conn_start");
    let pool = my::Pool::new("mysql://mysqlroot:12345678@localhost:3306").unwrap();
    let mut conn = pool.get_conn().unwrap();
    let mut p =P::new();
    let selected_rows = conn.exec_first("SELECT * FROM test.t_user", ()).map(|row:Option<(i32, String, i32, String)>| {
        // println!("dbrow:{:?}",result);
       let mut pp =  row.map(|(id,name,age,address)| P {
            id:id,
            name:name,
            age:age,
            address:address
        });
        let ppp = pp.unwrap();
        println!("closefn:{:?}",&ppp);
        p.id=ppp.id;
        p.name=ppp.name;
        p.age=ppp.age;
        p.address=ppp.address;
        // p.id=id;
        // p.name=name;
        // p.age=age;
        // p.address=address;

    }).unwrap();
    println!("outFun:{:?}",p);
    println!("outFun-json-str:{}",serde_json::to_string(&p).unwrap());
    let selected_rows :Option<(i32, String, i32, String)> = conn.exec_first("SELECT * FROM test.t_user", ()).unwrap();
    println!("{:?}",selected_rows);



    let resu:Vec<(i32, String, i32, String)>= conn.exec("select * from test.t_user_1",()).unwrap_or_else(|_|{vec!{(0,"".to_string(),0,"".to_string())}});
    for v in &resu{
        println!("vec-for-earch{:?}",v);
    }
    println!("{:?}",resu);
    println!("mysql_Test_conn_end");
}



     //match query_result {            Ok(result) => {                result            }            Err(_) => {                None            }        }