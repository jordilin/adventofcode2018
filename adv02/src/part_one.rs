use std::collections::HashMap;

pub fn count_frequency(contents: &str) -> i32 {
    let mut totals = vec![0, 0];
    for line in contents.lines() {
        let (mut twos, mut threes) = (0, 0);
        let mut freq_repetition: HashMap<String, u32> = HashMap::new();
        for char in line.chars() {
            let char_str = char.to_string();
            if let Some(val) = freq_repetition.get(&char_str) {
                let new = val + 1;
                match new {
                    2 => {
                        twos += 1;
                        freq_repetition.insert(char_str, new);
                    }
                    3 => {
                        threes += 1;
                        twos -= 1;
                        freq_repetition.insert(char_str, new);
                    }
                    _ => {
                        // shows more than three times, take one from threes.
                        threes -= 1;
                    }
                }
            } else {
                freq_repetition.insert(char_str, 1);
            }
        }
        if twos > 1 {
            twos = 1
        }
        if threes > 1 {
            threes = 1
        }
        totals[0] += twos;
        totals[1] += threes;
    }
    totals[0] * totals[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(12, count_frequency(input));
    }

    #[test]
    fn test_one_twos() {
        let input = "aabcdd\nabcdeee";
        assert_eq!(1, count_frequency(input));
    }
}
