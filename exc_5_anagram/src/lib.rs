use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut word_vec: Vec<_> = word.chars().collect();
    word_vec.sort();
    let mut return_set: HashSet<&'a str> = HashSet::new();
    for list in possible_anagrams {
        if word != list.to_lowercase() {
            println!("Word : {word}");
            println!("List : {list}");
            let mut list_vec: Vec<_> = list.to_lowercase().chars().collect();
            list_vec.sort();
            if list_vec == word_vec {
                return_set.insert(list.to_owned());
            }
        }
    }
    return_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_matches() {
        let word = "diaper";
        let inputs = &["hello", "world", "zombies", "pants"];
        let output = anagrams_for(word, inputs);
        let expected = HashSet::from_iter([]);
        assert_eq!(output, expected);
    }
    #[test]
    fn detects_two_anagrams() {
        let word = "solemn";
        let inputs = &["lemons", "cherry", "melons"];
        let output = anagrams_for(word, inputs);
        let expected = HashSet::from_iter(["lemons", "melons"]);
        assert_eq!(output, expected);
    }
    #[test]
    fn detects_multiple_anagrams_with_different_case() {
        let word = "nose";
        let inputs = &["Eons", "ONES"];
        let output = anagrams_for(word, inputs);
        let expected = HashSet::from_iter(["Eons", "ONES"]);
        assert_eq!(output, expected);
    }
    #[test]
    fn words_are_not_anagrams_of_themselves() {
        let word = "BANANA";
        let inputs = &["BANANA"];
        let output = anagrams_for(word, inputs);
        let expected = HashSet::from_iter([]);
        assert_eq!(output, expected);
    }
}
