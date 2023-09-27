use std::io::{self, Write};

use clap::Parser;
use rand::Rng;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
struct Args {
    /// Find the most common k words
    #[arg(short = 'n', long = "num-simulations", default_value = "1000000")]
    num_simulations: u32,
}

fn main() -> Result<()> {
    // Game 1:
    // There is a 20-sided die on the table with a 1 facing up.
    // You will be able to play for 100 rounds.
    // Your options for each round are:
    // - get the amount of money the die is showing
    // - reroll
    // What is your best strategy and how much do you expect to win using it?
    //
    // Game 2:
    // Now, when you take money, you take the die off the table (and have to now use 
    // a round to re-roll)
    //
    // Game 3:
    // Similar to Game 1, but now, when you take money, the casino gets to choose 
    // whether they want to re-roll or not (it does NOT take up a round if they 
    // choose to re-roll.)

    let args = Args::parse();

    for take_if_equal_or_greater_than in 1..21 {
        let mut total_winnings = 0;
        for _ in 0..args.num_simulations {
            let mut winnings = 0;
            let mut dice = 1;

            for _ in 0..100 {
                if dice >= take_if_equal_or_greater_than {
                    winnings += dice;
                } else {
                    dice = rand::thread_rng().gen_range(1..21);
                }
            }
            total_winnings += winnings;
        }
        let avg_winnings = total_winnings / args.num_simulations;
        writeln!(
            io::stdout(),
            "Take >={}, Winnings: {}",
            take_if_equal_or_greater_than,
            avg_winnings
        )?;
    }
    Ok(())
}
