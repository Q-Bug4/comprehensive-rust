fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        // word is &str type
        println!("word: {word}");
    }

    for word in v {
        // word is String type
        println!("word: {word}")
    }
}