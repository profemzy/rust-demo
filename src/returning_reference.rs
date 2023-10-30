pub fn return_reference(){
    let s = "New Brunswick";
    let r = get_first_word(&s);
    let status = get_score_message(60);
    println!("{}", r);
    println!("You {}", status);
}

fn get_first_word(s: &str) -> &str {
    let mut pos = 0;
    for ch in s.chars() {
        if ch == ' ' {
            break;
        }
        pos += 1
    }
    &s[..pos]
}

fn get_score_message(score: i32) -> &'static str {
    if score >= 50 {"PASSED"} else { "FAILED" }
}
