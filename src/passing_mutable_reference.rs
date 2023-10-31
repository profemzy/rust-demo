pub fn do_it() {
    println!("\nIn demo_passing_mutable_references::do_it()");

    let mut n = 42;
    let mut s = String::from("hello");

    some_func(&mut n, &mut s); // mutably borrows n and s

    n += 1_000_000;
    s.push_str("Nice!!");

    println!("n: {}", n);
    println!("s: {}", s);
}

fn some_func(i_param: &mut i32, s_param: &mut String){
    println!("Values Initially: {}, {}", i_param, s_param );
    *i_param += 10;
    s_param.push_str(" world👍🏻🤛🏾");
    println!("Values Afterwards: {}, {}", i_param, s_param );
}