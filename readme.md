# iChoose-Beta 0.1.2
## GitHub Author: cp-daniel-mccoy

iChoose is a simple program that randomly selects from a list of options for you.

This program is written in rust and is used to demonstrate basic rust functionality including user input, basic crate usage, loops, vectors, and other basics.

To use this program:
> **cargo run**

How this program works:

* 1.) It will ask you to enter some options (ex. places to eat). Enter each option line by line.
* 2.) Each time you enter a line, it will check to see if it's empty or if you typed "done".
l. 3.) It will add each line of valid input into a vector until "done" is entered.
l. 4.) It then generates a random number between 0 and the length of your options list.
l. 5.) Forces the data types between i32 and usize to use the generated number to reference the vector.
l. 6.) Displays a random selection from your list.

Planned Updates:

l. 1.) Better error handling.
l. 2.) Wrap in a loop to ask if you want to run again or exit.
l. 3.) Some kind of frontend.
l. 4.) A list loading/saving system to select from pre-existing lists.

Disclaimer:

At the time of creation this program was one of my first written in Rust. Therefore, this program is subject to errors, inefficiencies, and experimental changes throughout development.

I hope you enjoy using or modifying this, or that you learned or valued from it in some way or another.

GitHub Author: cp-daniel-mccoy
