use std::io::{self, Write, Read};

use rand::Rng;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()>{
    // Game 1: 
    // There is a 20-sided die on the table with a 1 facing up. 
    // You will be able to play for 100 rounds.
    // Your options for each round are:
    // - get the amount of money the die is showing
    // - reroll

    let cmd = std::env::args();
    let args = cmd.into_iter().collect::<Vec<_>>();
    let num_simulations: u32 = args[1].parse()?;
    // let mut input = String::new();
    // io::stdin().read_line(&mut input)?;
    // let num_simulations: u32 = input.trim().parse()?;

    for take_if_equal_or_greater_than in 1..20 {
        let mut total_winnings = 0;
        for _ in 0..num_simulations {
            let mut winnings = 0;
            let mut dice = 1;

            for _ in 0..100 {
                if dice > take_if_equal_or_greater_than {
                    winnings += dice;
                } else {
                    dice = rand::thread_rng().gen_range(1..21);
                }
            }
            total_winnings += winnings;
        }
        let avg_winnings = total_winnings / num_simulations;
        writeln!(io::stdout(), "Take >{}, Winnings: {}", take_if_equal_or_greater_than, avg_winnings)?;
    }
    Ok(())
}
