fn main() {
    let text_pig_latin = to_pig_latin("Arthur ,Hello , world!");
    println!("{text_pig_latin}");
}

fn to_pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|w| to_pig_latin_word(w))
        .fold(String::from(""), |mut acc, w| {
            acc.push_str(&w);
            acc.push_str(" ");
            acc
        })
}

fn to_pig_latin_word(w: &str) -> String {
    let word_chars: Vec<char> = w.chars().collect();

    let mut new_word = String::from("");
    if word_chars[0].is_alphabetic() {
        translate_alphabetic_word(&word_chars, &mut new_word, w);
    } else if word_chars[0].is_ascii_punctuation() {
        new_word.push(word_chars[0]);

        if word_chars.len() > 1 {
            new_word.push_str(&to_pig_latin_word(&w[1..]));
        }
    }
    new_word
}

fn translate_alphabetic_word(word_chars: &Vec<char>, new_word: &mut String, w: &str) {
    let vowels = ['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
    let (end_word_index, end_word_char) = get_end_word_refs(w);

    if vowels.contains(&word_chars[0]) {
        new_word.push_str(&w);
        new_word.push_str("-hay");
    } else {
        new_word.push_str(&w[1..end_word_index]);
        new_word.push_str("-");
        new_word.push(word_chars[0]);
        new_word.push_str("ay");
        new_word.push_str(end_word_char);
    }
}

fn get_end_word_refs(w: &str) -> (usize, &str) {
    let (end_word_index, end_word_char) = if w.ends_with(|c: char| c.is_ascii_punctuation()) {
        (w.len() - 1, &w[w.len() - 1..])
    } else {
        (w.len(), "")
    };
    (end_word_index, end_word_char)
}
