#[derive(Debug)]
pub  enum IpAddKind{
    V4,
    V6(String)
}
#[derive(Debug)]
pub struct IpAddr{
    pub kind :IpAddKind,
    pub address:String,
}



pub fn value_in_ip_add_kind(ip_add_kind:IpAddKind) -> u8 {
    match ip_add_kind{
        IpAddKind::V4 => 1,
        IpAddKind::V6(str)=>{
            println!("str:{}",str);
            return 2
        },
    }
}


pub fn optionTest(){
    //Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //if let 使用
    let some_and_value = Some(4);
    if let Some(3) = some_and_value{
        println!("three");
    } else {
        println!("qita");
    }

}


pub fn option_panic_Test(){

        panic!("123456");
}



fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

