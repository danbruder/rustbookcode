struct User {
    username: String,
}

fn main() {
    let s = String::from("hey");
    let user = User {
        username: String::from("Dan"),
    };
    println!("{}", first_word(&s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
