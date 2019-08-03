// strings.rs

fn main() {
    let question = "How are you ?"; // &str type
    let person: String = "Bob".to_string(); // String type
    let hello = String::from("สวัสดี");

    println!("{}! {} {}", hello, question, person);
}