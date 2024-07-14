use std::io;

fn main() {
    println!("Input a sentence containing the word Nemo, or not ;): ");

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    find_nemo(sentence);
}

fn find_nemo(sentence: String) {
    let mut count: i32 = 1;
    let mut found_nemo_bool = false;
    for word in sentence.split_whitespace() {
        if word == "Nemo" {
            found_nemo_bool = true;
            println!("I found Nemo at {}!", count);
        }
        count = count + 1;
    }

    if found_nemo_bool == false {
        println!("I can't find Nemo :(");
    }
}
