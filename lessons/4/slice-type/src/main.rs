fn main() {
    let a = String::from("hello world");
    let result = first_word(&a);
    println!("The first word of '{}' is {}.", a, result);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // returns the index of the first space
        // if item == b' ' {
        //     return i;
        // }
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

