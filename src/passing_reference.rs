pub fn do_it() {
    println!("\nIn passing_values::do_it()");

    let n = 60;
    let s  = "hello";

    some_func(&n, &s);
    some_func1(&n, &s);

    println!("n: {}", n);
    println!("s: {}", s.to_uppercase());
}

fn some_func(my_int: &i32, my_string: &str) {
    println!("In some_func, my_int is {} located at memory address {:p}", my_int, my_int);
    println!("In some_func, my_string is {} located at memory address {:p}", my_string.to_uppercase(), my_string);
}

fn some_func1(i_param: &i32, s_param: &str){
    if *i_param >= 50 {
        println!("{}, {}, PASS 😀", *i_param, s_param.to_uppercase());
    } else {
        println!("{}, {}, FAIL 😢", *i_param, s_param.to_lowercase());
    }
}