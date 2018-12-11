use std::collections::HashMap;

pub fn non_overlapped_claim(squares: &HashMap<(u32, u32), Vec<u32>>) -> u32 {
    let singles = squares
        .values()
        .filter(|v| v.len() == 1)
        .map(|v| v[0])
        .collect::<Vec<u32>>();

    let overlapped = squares
        .values()
        .filter(|v| v.len() > 1)
        .flatten()
        .collect::<Vec<&u32>>();

    for s in singles {
        let mut alone = true;
        for o in &overlapped {
            if s == **o {
                alone = false;
                break;
            }
        }
        if alone {
            return s;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input: HashMap<(u32, u32), Vec<u32>> =
            [((4, 4), vec![1, 2]), ((4, 2), vec![3]), ((4, 3), vec![1])]
                .iter()
                .cloned()
                .collect();
        assert_eq!(3, non_overlapped_claim(&input))
    }
}
