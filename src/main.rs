use clap::Parser;

#[derive(Parser)]
#[command(name = "zloopctl")]
struct Args {
    #[arg(short, long)]
    help: bool,
}

fn main() {
    println!("Hello, world!");
}
