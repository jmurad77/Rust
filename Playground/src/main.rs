fn main() {
    println!("Hard coded Hello");
    let mut message = String::from("HELLO WORLD MY FRIENDS");
    message.push('.');
    message.push_str(" I Added this Message To the End!");
    println!("This is a message from a string var: {}", message);
}
