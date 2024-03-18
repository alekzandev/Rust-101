use std::io;

fn get_words(sentence: String) -> Vec<String> {
    let trimmed_sentence = sentence.trim();
    let words: Vec<String> = trimmed_sentence.split_whitespace().map(|s| s.to_string()).collect();
    words
}
// return the longest word in the sentence as integer
fn get_longest_word(words: Vec<String>) -> (usize, String) {
    let mut longest: usize = 0;
    let mut longest_word = String::new();
    for word in words {
        if word.len() > longest {
            longest = word.len();
            longest_word = word;
        }
    }
    (longest, longest_word)
}

fn main() {
    let sentence = "the  quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    //iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel! -> {}", c),
            _ => continue,
        }
    }

    // Split and collect into a vector
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);
    //let words: Vec<&str> = sentence.split(' ').collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    println!("Enter a sentence: ");
    let mut user_sentence = String::new();
    io::stdin().read_line(&mut user_sentence).expect("Failed to read line");
    
    let words = get_words(user_sentence);
    match words.len() {
        0 => println!("No words found"),
        _ => {
            let (longest, longest_word) = get_longest_word(words);
            println!("The longest word is '{}' with {} characters", longest_word, longest);
        }
    }
}
