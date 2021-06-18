use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

pub fn rus() {

  let a =  File::open("aaa.text");
  let bfile = match a {
      Ok(file)=>file,
      Err(error)=>  {
          panic!("文件打开错误{:?}",error);
      },
  };

}


/**
*看到一个 unwrap 调用 panic!
*/
pub fn rus_unwrap(){
    let a =  File::open("aaa.text").unwrap();
}

//允许我们选择 panic! 的错误信息：expect
pub fn rus_expect(){
    let a =  File::open("aaa.text").expect("");
}

pub fn read_from_file()->Result<String,io::Error> {

    let f = File::open("hello.text");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s =String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}
/**
 * ?用于传播错误简写
 *   ? 运算符可被用于返回 Result 的函数
 */
pub fn read_from_file_jianxie()->Result<String,io::Error> {

    let mut f = File::open("hello.text")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
