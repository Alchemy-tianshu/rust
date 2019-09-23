//全局变量
fn main() {
    //主函数
    let var1 = 1;
    let bool1 = true;
    let str1 = "string";
    println!("{},{},{}",var1,bool1,str1);
    let string1 = "string".to_string();
    println!("{}",string1);
    //let float1 = 1.0;
    //let char1 = "天";
    //println!("{},{}"float1,char1);
    //let tuple = (1,);
    
    //定义三个元素的元组
    let mut var1:(u8,char,i32) = (1,'1',3);
    //通过“变量.索引”的形式访问指定元素
    println!("{}",var1.0); // ---->输出数字1

    let arr:[i32;3] = [1,2,3];
    println!("{}",arr[0]); //----->输出1
}
