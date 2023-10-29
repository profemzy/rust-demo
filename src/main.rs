use crate::passing_reference::do_it_with_ref;
use crate::passing_values::do_it;

mod passing_values;
mod passing_reference;

fn main() {
   demo_if();
    match_something();
    do_it();
    do_it_with_ref()
}

fn demo_if() {

    let age = 18;

    let msg = if age >   18 {"You are an OG and can vote"} else if age == 18  { "Welcome new voter, you are allowed to vote" } else { "You cannot vote" };

    println!("{}", msg)
}

fn match_something() {

    let num =5;
    let temp = 4;

    let day_of_week = match num {
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        7 => "Saturday",
        _ => "No Match"
    };

    println!("Day of the week is: {}", day_of_week);

    let weather_status = match temp {
        temp if temp > 21 => "it's a hot day",
        temp if temp == 21 => "it's a warm day",
        _ => "it's a cold day"
    };

    println!("{}", weather_status)
}


