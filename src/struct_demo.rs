//
// fn main(){
//     let mut r = Rectangle{
//         width:20,
//         height:20
//     };
//     r.setHeight(300);
//     println!("r.area:{}",r.area());
//
//     let square = Rectangle::square(300);
//     println!("square：{}",square.height);
// }
//
// #[derive(Debug)]
// struct Rectangle {
//     width:u32,
//     height:u32,
// }
//
// impl Rectangle{
//     fn square(size:u32)-> Rectangle{
//         return  Rectangle{ width:size,height:size};
//     }
//
//     fn setHeight(&mut self,height:u32){
//         self.height = height;
//     }
//
//     fn area(&self) -> u32{
//         self.height* self.width
//     }
// }