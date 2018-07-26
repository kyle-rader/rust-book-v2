fn main() {
    let s = String::from("Hello Ownership :)");
    println!("{}", s);

    let mut s2 = s.clone();
    s2.push_str(" (from s2)");

    takes_ownership(s); // s is now invalid bceause we didn't regain ownership
    let mut s2 = take_and_give_back(s2);

    println!("And we have '{}' back in main", s2);
    print_len(&s2);

    for _i in 0..10 {
        add_star(&mut s2);
    }
    println!("After add_star loop s2 is: '{}'", s2);
}

fn takes_ownership(a_string: String) {
    println!("Now 'takes_ownership' owns '{}'", a_string);
}

fn take_and_give_back(a_string: String) -> String {
    println!("Now 'take_and_give_back' owns '{}'", a_string);
    a_string
}

fn print_len(a_string_ref: &String) {
    println!("'print_len' can just use {} to print it's length which is: {}",
        a_string_ref,
        a_string_ref.len());
}

fn add_star(s: &mut String) {
    s.push_str("*");
}