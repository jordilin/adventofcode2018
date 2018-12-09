
fn hamming_distance(input1: &str, input2: &str) -> u32 {
    input1
        .chars()
        .zip(input2.chars())
        .filter(|&(c1, c2)| c1 != c2)
        .map(|_| 1)
        .sum()
}

pub fn correct_box_id(input: &str) -> String {
    let entries = input.split("\n").collect::<Vec<&str>>();
    for (i, entry) in entries.iter().enumerate() {
        for (k, other) in entries.iter().enumerate() {
            if i == k {
                continue;
            }
            if hamming_distance(entry, other) == 1 {
                return entry
                    .chars()
                    .zip(other.chars())
                    .filter(|&(c1, c2)| c1 == c2)
                    .map(|(c1, _)| c1)
                    .collect();
            }
        }
    }
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_same_word_is_zero() {
        let input1 = "fghij";
        let input2 = "fghij";
        assert_eq!(0, hamming_distance(input1, input2));
    }

    #[test]
    fn test_distance_is_one() {
        let input1 = "fghij";
        let input2 = "fguij";
        assert_eq!(1, hamming_distance(input1, input2));
    }

    #[test]
    fn test_example() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!("fgij".to_string(), correct_box_id(input));
    }

}
