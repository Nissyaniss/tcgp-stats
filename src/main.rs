use std::fs::read_to_string;

use comfy_table::Table;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Display the current drop rate table of the game
    #[clap(short, long)]
    drop_rate: bool,
}

fn main() {
    let args = Args::parse();

    if args.drop_rate {
        display_drop_rate();
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