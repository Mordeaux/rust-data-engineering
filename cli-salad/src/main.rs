use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
    #[clap(short, long, num_args(0..))]
    fruits: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let fruits = opts.fruits;

    // Get the number of fruits the user requested
    let mut num_fruits = opts.number;

    // Create the fruit salad
    let mut fruit_salad = create_fruit_salad(num_fruits, &fruits);
    num_fruits += fruits.len();

    // sort fruit salad alphabetically
    fruit_salad.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits, fruit_salad
    );
}
