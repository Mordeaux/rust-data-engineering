/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Take user input to add more fruits to the front of the list
    println!("Enter a fruit to add to the front of the list:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    fruit.push_front(input);

    // Now do the same for the back of the list
    println!("Enter a fruit to add to the back of the list:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    fruit.push_back(input);

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
    // Select a random winning fruit using SliceRandom
    let mut rng = thread_rng();
    let fruit: Vec<_> = fruit.into_iter().collect();
    let winning_fruit = fruit.choose(&mut rng).unwrap();
    println!("The winning fruit is: {}", winning_fruit);

    // Convert back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Remove a fruit from the front of the list, print it, and then print the new list
    let removed_fruit = fruit.pop_front().unwrap();
    println!("Removed fruit: {}", removed_fruit);
    let fruit: Vec<_> = fruit.into_iter().collect();
    println!("New fruit list: {:?}", fruit);

    // Remove a fruit from the back of the list, print it, and then print the new list
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();
    let removed_fruit = fruit.pop_back().unwrap();
    println!("Removed fruit: {}", removed_fruit);
    let fruit: Vec<_> = fruit.into_iter().collect();
    println!("New fruit list: {:?}", fruit);
}
