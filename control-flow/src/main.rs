
fn main() {
    // variables are immutable by default,
    // mark them as mut to be able to reassign them
    let mut x = 5;
    println!("The value of x is: {x}");

    x =  6;
    println!("The value of x is: {x}");
    another_function(x);

    // consts can be declared something like this
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    {
        println!("See this function can also access the const");
        // you cannot overwrite const's ever
        // const THREE_HOURS_IN_SECONDS = 200;
        println!("{}",THREE_HOURS_IN_SECONDS);
    }
}

fn another_function(x: i32){
    println!("The square of x is {}",x*x)
}
