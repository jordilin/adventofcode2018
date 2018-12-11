use std::collections::HashMap;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    fn new() -> Claim {
        Claim {
            id: 0,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }
}

// A regex would be better...
fn get_coord(line: &str) -> Option<Claim> {
    //#1 @ 1,3: 4x4
    let fields = line.split(" ").collect::<Vec<&str>>();
    if fields.len() != 4 {
        return None;
    }
    let f_length = fields.len();
    let mut claim = Claim::new();
    let wh = fields[f_length - 1].split("x").collect::<Vec<&str>>();
    claim.w = wh[0].parse::<u32>().unwrap();
    claim.h = wh[1].parse::<u32>().unwrap();
    let xy = fields[2].split(":").collect::<Vec<&str>>()[0]
        .split(",")
        .collect::<Vec<&str>>();
    claim.x = xy[0].parse::<u32>().unwrap();
    claim.y = xy[1].parse::<u32>().unwrap();
    claim.id = fields[0].split("#").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
    Some(claim)
}

pub fn get_squares(input: &str) -> HashMap<(u32, u32), Vec<u32>> {
    let lines = input.split("\n");
    let mut squares: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    for line in lines {
        if let Some(claim) = get_coord(line) {
            for i in claim.y..claim.y + claim.h {
                for j in claim.x..claim.x + claim.w {
                    let val = squares.entry((i, j)).or_insert(Vec::new());
                    val.push(claim.id);
                }
            }
        }
    }
    squares
}

pub fn overlapped_square_feet(squares: &HashMap<(u32, u32), Vec<u32>>) -> usize {
    squares.iter().filter(|(_, v)| v.len() > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(4, overlapped_square_feet(&get_squares(&input)))
    }
}
