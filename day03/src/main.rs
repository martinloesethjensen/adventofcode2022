use std::fs;

fn main() {
    let contents =
        fs::read_to_string("challenge.txt").expect("Should have been able to read the file");

    let alphabet_upper = (b'A'..=b'Z') // Start as u8
        .filter_map(|c| {
            let c = c as char;
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let alphabet_lower = (b'a'..=b'z') // Start as u8
        .filter_map(|c| {
            let c = c as char;
            if c.is_alphabetic() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let priority_lower = (1u32..=26).collect::<Vec<_>>();
    let priority_upper = (27u32..=52).collect::<Vec<_>>();

    println!("{:?}", alphabet_lower);
    println!("{:?}", alphabet_upper);
    println!("{:?}", priority_lower);
    println!("{:?}", priority_upper);



    let mut count = 0u32;

    for line in contents.split("\n").into_iter() {
        let line_len = line.len();
        let first_half = line.get(0..(line_len / 2)).unwrap();
        let second_half = line.get((line_len / 2)..line_len).unwrap();

        for item in first_half.chars().into_iter() {
            if second_half.contains(item) {
                let value = if item.is_lowercase() {
                    let index = alphabet_lower.iter().position(|&x| x == item).unwrap();
                    priority_lower[index]
                } else {
                    let index = alphabet_upper.iter().position(|&x| x == item).unwrap();
                    priority_upper[index]
                };
                count += value;
                break;
            }
        }
    }

    println!("{}", count);
}
