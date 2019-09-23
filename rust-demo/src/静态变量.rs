//主函数
fn main() {
    //println!("Hello, world!");
    // 定义病初始化一个变量var1
    //let var1 = 1;
    //打印格式化文本并在结尾输出换行
    //println!("{}",var1);

    //对不可变变量重新赋值--->会报错
    //var1 = 2;
   //println!("{}",var1);
    //定义可变变量 mut var1

    //let mut var1 = 1;
    //println!("{}",var1);//输出1
    //var1 = 2;
    //println!("{}",var1);//输出2
    
    //定义整形静态变量，静态变量定义时必须同时初始化并制定类型
    static VAR1:i32 = 0;
    //定义一个可变静态变量
    static mut VAR2:i32 = 0;

    //在unsafe作用域中读写VAR2
    unsafe {
        println!("{}",VAR2);//--->输出0
        VAR2 = 2;
        println!("{}",VAR2);// --->输出2
    }

    println!("{}",VAR1);  //----->输出0
}
