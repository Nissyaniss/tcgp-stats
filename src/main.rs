use std::fs::read_to_string;
use std::io::prelude::*;

use comfy_table::Table;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Display the current drop rate table of the game
    #[clap(short, long)]
    drop_rate: bool,

    #[clap(short, long, value_delimiter = ' ', num_args = 1..)]
    cards: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    if args.drop_rate {
        display_drop_rate();
    }

    if let Some(cards) = args.cards {
        let data = read_csv("./src/csv/genetic_apex.csv");
        let mut data: Vec<Vec<String>> = data.lines().map(|line| line.split(',').map(|s| s.to_string()).collect()).collect::<Vec<Vec<String>>>();

        for card in cards {
            if card == "all" {
                for line in data.clone() {
                    data[line[1].split_whitespace().last().unwrap().parse::<usize>().unwrap() - 1][0] = "TRUE".to_string();
                }
            } else {
                for line in data.clone() {
                    if line[2] == card {
                        data[line[1].split_whitespace().last().unwrap().parse::<usize>().unwrap() - 1][0] = "TRUE".to_string();
                        break;
                    }
                }
            }
        }

        let mut file = std::fs::File::create("./src/csv/genetic_apex.csv").unwrap();
        file.write_all(data.iter().map(|line| line.join(",") + "\n").collect::<String>().as_bytes()).unwrap();
    }
}

fn read_csv(path: &str) -> String {
    read_to_string(path).unwrap_or("Unable to read file".to_string())
}

fn display_drop_rate() {
    let drop_rate = read_csv("./src/csv/drop_rate.csv");

    let mut table = Table::new();

    table.set_header(drop_rate.lines().next().unwrap().split(','));

    for line in drop_rate.lines().skip(1) {
        let vec_line = line.split(',');

        table.add_row(vec_line);
    }

    println!("{}", table);
}