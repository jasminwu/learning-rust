fn fahrenheit_to_celsius(temp_fahr: f32) -> f32 {
    (temp_fahr - 32.0) * (5.0 / 9.0)
}

fn nth_fib_number(n: u32) -> u32 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        nth_fib_number(n - 1) + nth_fib_number(n - 2)
    }
}

fn twelve_days_of_xmas() {
    let numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "And a partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for n in 0..11 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            numbers[n]
        );

        if n == 0 {
            println!("A partridge in a pear tree");
        } else {
            for n_gifts in (0..n + 1).rev() {
                println!("{}", gifts[n_gifts]);
            }
        }
        println!("\n");
    }
}

fn main() {
    let fahr_temp = 32.0;
    let cels_temp = fahrenheit_to_celsius(fahr_temp);
    println!("{fahr_temp} degrees Fahrenheit is {cels_temp} degrees Celsius.\n");

    let n = 7;
    let nth_fib = nth_fib_number(n);
    println!("The {n}th Fibonnacci Number is {nth_fib}\n");

    twelve_days_of_xmas();
}
