pub fn do_it() {
    let mut s = String::from("New");
    let r = some_func(&mut s);

    println!("{}", r);
}

fn some_func(s: &mut String) -> &mut String {
    s.push_str(" Brunswick");
    s
}
