use std::fs;

fn main() {
    let contents =
        fs::read_to_string("challenge.txt").expect("Should have been able to read the file");

    let splitted = contents.split("\n\n").collect::<Vec<_>>();

    println!("{:?}", splitted);

    let mut folded = splitted
        .into_iter()
        .map(|x| {
            String::from(x)
                .split("\n")
                // parse to i32 and fold values
                .fold(0i32, |acc, x| acc + x.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    println!("{:?}", folded);

    folded.sort_by(|a, b| b.cmp(a));

    let sorted_top_three: Vec<i32> = folded.into_iter().take(3).collect();

    println!("Top 1: {}", sorted_top_three[0]);

    let top_three_sum: i32 = sorted_top_three.into_iter().sum();

    println!("Sum of top 3: {}", top_three_sum);
}
