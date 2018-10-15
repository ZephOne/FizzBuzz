fn main() {
    let fizz = |x: u8| x % 3 == 0;

    let buzz = |x: u8| x % 5 == 0;

    for i in 0..200 {
        if fizz(i) && buzz(i) {
            println!("FizzBuzz")
        } else if fizz(i) {
            println!("Fizz")
        } else if buzz(i) {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}
