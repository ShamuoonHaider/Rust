fn main() {
    let sum = 5 + 10;
    println!("the sum is{sum}");

    let difference = 95.9 - 2.8;
    println!("the difference is {difference}");

    let division = 5 / 9;
    println!("The division is {division}");

    let truncated = -19 / 7;
    println!("The truncated result is {truncated}");

    let reminder = 4 % 6;
    println!("The reminder is {reminder}");

    // Boolean types
    //
    // {
    //   let t: bool = true;

    //   let f: bool = false
    // }

    {
        let tup: (i32, f64, u8) = (500, 6.5, 10);
        println!("the is the output of a tuple: {:?}", tup);
    }
}
