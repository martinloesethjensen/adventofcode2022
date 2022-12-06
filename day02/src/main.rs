use std::fs;

fn main() {
    let contents =
        fs::read_to_string("challenge.txt").expect("Should have been able to read the file");

    let splitted = contents.split("\n").collect::<Vec<_>>();

    println!("{:?}", splitted);

    let mut count = 0u32;

    for round in splitted.into_iter() {
        let round_choices = round.split(" ").collect::<Vec<_>>();
        let opponent_choice = round_choices[0];
        let your_choice = match round_choices[1] {
            "X" => get_loss_choice(opponent_choice),
            "Z" => get_win_choice(opponent_choice),
            _ => opponent_choice,
        };

        let rock = "A";
        let paper = "B";
        let scissor = "C";

        // paper:   B
        // rock:    A
        // scissor: C
        let winning_possibilities = vec!["A B", "B C", "C A"];
        let _loosing_possibilities = vec!["B A", "A C", "C B"];

        let choice_value: u32 = match your_choice {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            &_ => 0,
        };

        let composed = format!("{opponent_choice} {your_choice}");

        let result_value = if winning_possibilities
            .into_iter()
            .any(|x| x == format!("{opponent_choice} {your_choice}"))
        {
            println!("🥳 Won with this round {}", composed);
            6
        } else if your_choice == opponent_choice {
            println!("😐 Draw with this round {}", composed);
            3
        } else {
            println!("🥲 Lost with this round {}", composed);
            0
        };

        count += choice_value + result_value;
    }

    println!("{}", count);
}

fn get_win_choice(choice: &str) -> &str {
    return match choice {
        "A" => "B",
        "B" => "C",
        &_ => "A",
    };
}

fn get_loss_choice(choice: &str) -> &str {
    return match choice {
        "A" => "C",
        "B" => "A",
        &_ => "B",
    };
}
