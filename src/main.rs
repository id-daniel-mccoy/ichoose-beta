#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::io::{BufRead};
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use rand::prelude::*;
#[allow(unused_imports)]
use rand::Rng;

#[allow(non_snake_case)]
fn main() {
    askMe();
}

fn space() {
    println!("");
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn askMe() {
    space();
    println!("This program will give you a list of your entered selections.");
    println!("In this scenario, we are going to use foods.");
    space();
    println!("Enter your favorite foods line by line, when you are done, just type 'done'");
    space();
    let mut theList = Vec::new();
    let mut doneYet = false;
    let stdin = io::stdin();
    while doneYet == false {
        let toBeAdded = stdin.lock().lines().next().unwrap().unwrap();
        if toBeAdded.to_uppercase() == "DONE" {
            doneYet = true;
        } else {
            space();
            println!("Your selection has been added to the list.");
            space();
            theList.push(toBeAdded);
        }
    }

    let theRand:i32 = random();
    let mut vecLength:i32 = theList.len().try_into().unwrap();
    vecLength -= 1;
    let mut rng = rand::thread_rng();
    let selection:usize = rng.gen_range(0..vecLength).try_into().unwrap();
    let autoChoice = &theList[selection];
    space();
    space();
    space();
    println!("{}{}", "We have chosen for you: ", autoChoice);
    space();
    println!("Thanks for using this program.");
    println!("Author: Daniel McCoy (GitHub: cp-daniel-mccoy)");
    space();
}