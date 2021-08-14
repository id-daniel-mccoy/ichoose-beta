use std::io;
use std::io::{BufRead};
use std::convert::TryInto;
use rand::prelude::*;
use rand::Rng;

#[allow(non_snake_case)]
#[quit::main]
fn main() {
    askMe();
}

fn space() {
    println!("");
}

#[allow(non_snake_case)]
fn askMe() {
    space();
    println!("This program will ask you for a list of options and then choose one for you.");
    println!("Enter options like places to eat or things to cook.");
    space();
    println!("Enter each option line by line. When you are done, just type 'done'");
    space();
    let mut theList = Vec::new();
    let mut doneYet = false;
    let stdin = io::stdin();
    while doneYet == false {
        let toBeAdded = stdin.lock().lines().next().unwrap().unwrap();

        if toBeAdded.to_uppercase() == "DONE" {
            doneYet = true;
        } else if toBeAdded == "" {
            space();
            println!("Error: You did not enter any input.");
            println!("The program has crashed.");
            println!("This is a known error and will handled in a future update.");
            space();
            quit::with_code(0);
        } else {
            space();
            println!("Your selection has been added to the list.");
            space();
            theList.push(toBeAdded);
        }
    }

    let _theRand:i32 = random();
    let mut vecLength:i32 = theList.len().try_into().unwrap();

    if vecLength == 0 {
        space();
        println!("Error: You did not enter any input.");
        println!("The program has crashed.");
        println!("This is a known error and will be handled in a future update.");
        space();
        quit::with_code(0);
    }

    if vecLength == 1 {
        space();
        println!("Error: Only 1 option to choose from.");
        println!("The program has crashed.");
        println!("This is a known error and will be handled in a future update.");
        space();
        quit::with_code(0);
    }

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
