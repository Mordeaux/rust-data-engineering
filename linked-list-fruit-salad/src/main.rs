/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;

// A function that will add an element to a linked list at an arbitrary index
fn insert<T>(list: &mut LinkedList<T>, index: usize, element: T) -> &mut LinkedList<T> {
    // Get the length of the list
    let len = list.len();

    // If the index is greater than the length of the list, then return
    if index > len {
        panic!("Index out of bounds");
    }

    // If the index is 0, then push the element to the front of the list
    if index == 0 {
        list.push_front(element);
        return list;
    }

    // If the index is equal to the length of the list, then push the element to the back of the list
    if index == len {
        list.push_back(element);
        return list;
    }
    let mut split = list.split_off(index);
    list.push_back(element);
    list.append(&mut split);

    list
}

// A function that will remove an element from a linked list at an arbitrary index
fn remove<T>(list: &mut LinkedList<T>, index: usize) -> T {
    // Get the length of the list
    let len = list.len();

    // If the index is greater than the length of the list, then return
    if index >= len {
        panic!("Index out of bounds");
    }

    // If the index is 0, then pop the element from the front of the list
    if index == 0 {
        return list.pop_front().unwrap();
    }

    // If the index is equal to the length of the list - 1, then pop the element from the back of the list
    if index == len - 1 {
        return list.pop_back().unwrap();
    }

    let mut split = list.split_off(index);
    let element = split.pop_front().unwrap();
    list.append(&mut split);

    element
}

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Take input from the user, first an index and then a fruit
    println!("Enter an index and a fruit:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let index = input.next().unwrap().parse::<usize>().unwrap();
    let fruit_name = input.next().unwrap();

    // Insert the fruit at the index, remember that fruit is a linked list and so it does not have an insert method
    // We need to use the LinkedList::insert method
    insert(&mut fruit, index, fruit_name);

    // Add fruits to the both ends of the list after shuffling
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

    let mut rng = thread_rng();
    let fruit: Vec<_> = fruit.into_iter().collect();
    let winner = fruit.choose(&mut rng).unwrap();
    println!("The winning fruit is {}", winner);

    // take an index from the user then remove that fruit from the list
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();
    println!("Enter an index to remove:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap();
    let removed = remove(&mut fruit, index);
    println!("Removed {}", removed);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
