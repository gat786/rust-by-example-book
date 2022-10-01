
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

    if_examples(12);

    looping_around();

    loop_with_labels();

    loop_with_conditions();
    
    looping_through_items();
}

fn another_function(x: i32){
    println!("The square of x is {}",x*x)
}

fn if_examples(_age: i32){
    let age = _age;

    let result = if age < 18 { "in-eligible" } else { "eligible" };

    println!("Current age is {age} and result is {result}")

}

fn looping_around(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // value that should be returned is placed after the break statement
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_with_labels(){
    let mut table_of = 1;

    'counting_up: loop {
        let mut multiplied_by = 1;
        loop{
            if table_of == 11 {
                break 'counting_up;
            }
            let result = table_of * multiplied_by;
            println!("{table_of} x {multiplied_by} = {result}");
            multiplied_by += 1;
            if multiplied_by == 11 {
                break;
            }
        }
        table_of += 1;
    }
}

fn loop_with_conditions(){
    let table_of = 27;
    let mut multiplied_by = 1;
    while multiplied_by < 11 {
        let result = table_of * multiplied_by;
        println!("{table_of} x {multiplied_by} = {result}");
        multiplied_by += 1;
    }
}

fn looping_through_items(){ 
    let a = [1,2,3,4,5,6,7,8,9,10];
    for x in a {
        let result = 99 * x;
        println!("99 x {x} = {result}")
    }
}
