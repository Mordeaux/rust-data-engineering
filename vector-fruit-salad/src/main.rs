/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::io;

fn main() {
    // A vector of Strings of names of 15 different fruits containing none of the fruits in the fruit vec below
    let hidden_fruits = vec![
        "Apricot".to_string(),
        "Banana".to_string(),
        "Blueberry".to_string(),
        "Cantaloupe".to_string(),
        "Grape".to_string(),
        "Guava".to_string(),
        "Kiwi".to_string(),
        "Lemon".to_string(),
        "Lime".to_string(),
        "Mango".to_string(),
        "Olive".to_string(),
        "Pineapple".to_string(),
        "Plum".to_string(),
        "Raspberry".to_string(),
        "Strawberry".to_string(),
    ];
    let mut fruit = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];
    println!("How many hidden fruits would you like to add to the salad:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num_hidden_fruits: usize = input.trim().parse().unwrap();
    // Add hidden fruits to the fruit salad
    for _ in 0..num_hidden_fruits {
        fruit.push(
            hidden_fruits
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
        );
    }

    println!("Enter fruits to add to the salad:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "" {
            break;
        }
        fruit.push(input.trim().to_string());
    }

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);
    let winning_fruit = fruit.choose(&mut rng).unwrap();

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
    println!("The winning fruit is {}", winning_fruit);
}
