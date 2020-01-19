fn main() {
    let text = String::from("There are no errors in the world without constants");
    println!("Original text: {:?}", text);
    println!("In Pig Latin it's: {:?}", translate(&text));
}

fn translate(text: &str) -> String {
    // The first consonant of each word is moved to the end of the word
    // and “ay” is added, so “first” becomes “irst-fay.” Words that start
    // with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”)
    let mut pig_text = String::new();
    let vowels = vec!('A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u', 'Y', 'y');
    for word in text.split_whitespace() {
        let mut pig_word = String::new();
        let mut ending = String::new();
        for (i, c) in word.chars().enumerate() {
            if !c.is_alphabetic() {
                continue;
            }
            if i == 0 {
                // decide ending based on the first letter
                if vowels.contains(&c) {
                    ending = "-hay".to_string();
                    pig_word.push(c);
                } else {
                    ending = format!("-{}ay", c.to_lowercase());
                }
            } else {
                pig_word.push(c);
            }
        }
        pig_word = format!("{}{} ", pig_word, ending);
        pig_text = format!("{}{}", pig_text, pig_word);
    }
    pig_text.trim().to_string()
}
