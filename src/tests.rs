#[cfg(test)]
mod tests {

    use crate::my_mod::struct_demo::Rectangle;
    use crate::my_mod::ip::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        //使用标准库的宏
        assert!(larger.can_hold(&smaller));
    }

    //assert_eq! 相等
    //assert_ne! 不相等
    #[test] //表明一个测试函数
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    //使用 should_panic 检查 panic
    #[test]
    #[should_panic(expected = "123456")]
    fn optionTest_test() {
        option_panic_Test();
    }


    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}