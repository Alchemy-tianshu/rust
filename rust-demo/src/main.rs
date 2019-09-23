fn main(){
    //创建空字符串
    let str:String = String::new();

    //从abc字面量字符串创建String类型
    let str1:String = String::From("abc");

    //从abc字面量字符串创建String类型，与上一句作用相同
    let str2:String = "abc".to_string();
    //以上字符串不可修改，均为只读，未加mut修饰符
}
