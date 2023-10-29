pub fn return_value(){
    let n = f1(); // receives copy
    println!("{}", n);
    let s = f2(); // receives ownership
    println!("{}", s);
    // let status = f3(45);
    // println!("{}", status);
}

fn f1() -> i32 {
    let n = 44;
    return n; // returns a copy
}

fn f2() -> String {
    let s = String::from("Chilliwack");
    return s; // returns ownership
}
