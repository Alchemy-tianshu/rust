//全局变量
static VAR1:i32 = 0;
fn main() {
    //主函数
    println!("{}",VAR1);  //---->输出0
    test();
}

fn test(){
    println!("{}",VAR1);//--->输出0
}
