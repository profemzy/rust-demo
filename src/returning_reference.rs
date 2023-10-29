pub fn return_reference(){
    let s = "Satellite";
    let r = f1(&s);
    println!("{}", r);
}

fn f1(s_param: &str) -> &str {
    return s_param;
}
