pub fn do_it(){
    let n = f1(); // receives copy
    println!("{}", n);
    let s = f2(); // receives ownership
    println!("{}", s);
}

fn f1() -> i32 {
    let n = 44;
    return n; // returns a copy
}

fn f2() -> String {
    let s = String::from("Chilliwack");
    return s; // returns ownership
}
