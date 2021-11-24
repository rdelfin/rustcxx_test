use cards_rs::Suit;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "cards_cli")]
struct Opt {
    #[structopt(short, long)]
    card_name: String,
}

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", Suit::from(opt.card_name));
}
