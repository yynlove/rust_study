pub fn test_suoyouquan() {
    //s值移入到函数中 之后不在有效
    let s = String ::from("hello");
    takes_ownership(s);
    //返回值转移所有权
    let s2 = String::from("abc");
    let s3 = takes_and_give_back(s2);
    println!("s3:{}",s3);
    //转移返回值的所有权
    let s1 = String::from("word");
    let (s2, len) = calculate_length(s1);
    println!("s2:The length of '{}' is {}.", s2, len);

    let x =5; //x是copy的，所以后面可继续使用
    makes_copy(x);


}

fn takes_ownership(some_string:String){
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}",some_integer);
}

fn takes_and_give_back(str:String) -> String {
    str
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}
