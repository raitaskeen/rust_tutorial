#![allow(unused)]
pub mod guessgame;
pub mod array;

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


pub(crate) fn main() {
    print!("What is your name? ");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you ";
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line"); // Read line from user input
    println!("Taskeen Haider");
}