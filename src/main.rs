// fn main() {
//     //s值移入到函数中 之后不在有效
//     let s = String ::from("hello");
//     takes_ownership(s);
//     //返回值转移所有权
//     let s2 = String::from("abc");
//     let s3 = takes_and_give_back(s2);
//     println!("s3:{}",s3);
//     //转移返回值的所有权
//     let s1 = String::from("word");
//     let (s2, len) = calculate_length(s1);
//     println!("s2:The length of '{}' is {}.", s2, len);
//
//     let x =5; //x是copy的，所以后面可继续使用
//     makes_copy(x);
//
//
// }
//
// fn takes_ownership(some_string:String){
//     println!("{}",some_string);
// }
//
// fn makes_copy(some_integer:i32){
//     println!("{}",some_integer);
// }
//
// fn takes_and_give_back(str:String) -> String {
//     str
// }
//
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 返回字符串的长度
//     (s, length)
// }

#[derive(Debug)]
enum IpAddKind{
    V4,
    V6(String)
}
#[derive(Debug)]
struct IpAddr{
    kind :IpAddKind,
    address:String,
}



fn main(){
    //使用结构体和枚举定义数据
    let home = IpAddr{
        kind:IpAddKind::V4,
        address:String::from("127.0.0.1")
    };
    let loopback = IpAddr{
        kind:IpAddKind::V6(String::from("::1")),
        address:String::from("::1")
    };
    //使用match
    let b = value_in_ip_add_kind(home.kind);
    println!("v4:{}",b);

    let a = value_in_ip_add_kind(loopback.kind);
    println!("v6:{}",a);

    //Option<T>
    //let five = Some(5);
    //let six = plus_one(five);
    //let none = plus_one(None);


    //if let 使用
    let some_and_value = Some(4);
    if let Some(3) = some_and_value{
        println!("three");
    } else {
        println!("qita");
    }


}


fn value_in_ip_add_kind(ip_add_kind:IpAddKind) -> u8 {
    match ip_add_kind{
        IpAddKind::V4 => 1,
        IpAddKind::V6(str)=>{
            println!("str:{}",str);
            return 2
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
