fn main() {
    let number = 7;

    if number != 0 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let numner = if condition { 5 } else { 6 };

    println!("The value of number is: {numner}");
}
