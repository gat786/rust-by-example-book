fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // without ampersand it would transfer ownership that would make `s` unusable
    // for the rest of the program 
    // give_ownership(s);
    // println!("{}", s); 

    // it is essential that we pass it as a reference so that we are the owner
    // even after calling the function
    give_ownership(&s);

    println!("{}", s); 

    mutable_refs(&mut s);

    println!("value after mutating inside other function {}",s);
}

fn give_ownership(s1: &String){
    // let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // This doesn't work because we moved value to s1
    println!("{}, world!", s2); // This works
}

fn mutable_refs(some: &mut String){
    some.push_str(" english");
    println!("{}",some);
}

