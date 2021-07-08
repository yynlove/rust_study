mod my_mod;
mod tests;
pub use crate::my_mod::ip::{value_in_ip_add_kind,IpAddKind,IpAddr};
pub use crate::my_mod::suoyouquan::test_suoyouquan;
use crate::my_mod::shi::build;

fn main(){
    //使用结构体和枚举定义数据
    // let home = IpAddr{
    //     kind:IpAddKind::V4,
    //     address:String::from("127.0.0.1")
    // };
    // let loopback = IpAddr{
    //     kind:IpAddKind::V6(String::from("::1")),
    //     address:String::from("::1")
    // };
    //使用match
    // b = value_in_ip_add_kind(home.kind);
    //println!("v4:{}",b);

    //let a = value_in_ip_add_kind(loopback.kind);
    //println!("v6:{}",a);
    //所有权
    //test_suoyouquan();

    //第九章
    //panic!打印出一个错误信息，展开并清理栈数据
    //my_mod::jiu::rus();
    //unwrap result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    //my_mod::jiu::rus_unwrap();
    //expect 提供一个好的错误信息可以表明你的意图
    //my_mod::jiu::rus_expect();
    //错误传播
    //my_mod::jiu::read_from_file().unwrap();
    //错误传播简写？
    //my_mod::jiu::read_from_file_jianxie().unwrap();

    //第十章
    //泛型
    //my_mod::shi::build();
    //trait
    //my_mod::shi::summary_tweet_test();
    //trait+泛型
    //my_mod::shi::trait_fanxing();
    //泛型生命周期
    my_mod::shi::longest_test_success();








}

