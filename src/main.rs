mod my_mod;
pub use crate::my_mod::ip::{value_in_ip_add_kind,IpAddKind,IpAddr};
pub use crate::my_mod::suoyouquan::test_suoyouquan;

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
    //所有权
    test_suoyouquan();


}

