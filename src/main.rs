fn main() {
    let age = 6;

    println!("{}", get_age(age));

    let age = 14;

    println!("{}", get_age(age));

    let age = 24;

    println!("{}", get_age(age));
}

fn get_age(age: u8) -> String {
    if age > 18 {
        String::from("Adult")
    } else if age > 12 {
        String::from("Teenager")
    } else {
        String::from("Child")
    }
}
