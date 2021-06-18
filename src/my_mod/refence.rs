pub fn test(){
    let mut s = String::from("hello");
    let s1 = &mut s;
    println!("s1:{}",s1);

    change_reference(&mut s);
    println!("s2:{}",s);

}


pub fn change_reference(some_string: &mut String) {
    some_string.push_str(", world");
}


