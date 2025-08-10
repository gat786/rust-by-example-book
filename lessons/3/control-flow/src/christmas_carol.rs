pub fn print_carol(){
    let lines = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for a in 0..12{
        let day_no = a + 1;
        println!("[Verse {day_no}]");
        println!("On the {day_no} day of Christmas, my true love sent to me");
        for line_no in (0..a + 1).rev() {
            println!("{}", lines[line_no]);
        }
        println!("\n");
    }
}