# Advent Of Code 2023

My solutions for Advent of Code 2023 implemented in Rust

### Day 1

First introduction to Rust. My first impression is that the pervasive use of Option<> makes even the most trivial tasks, such as this one, very complicated.

### Day 2

Copilot is turning out to be relatively useful. It is pretty good at doing the boilerplate code and copy pasta. The boilerplate code is  especially helpful while Rust is new to me. I don't have to worry about all of the minute details.

### Day 3

Collections in Rust are pretty straightforward. Objects are still a bit of a mystery. 

### Day 4

Objects are still a mystery -- object vs. reference and move vs. copy, and wtf is borrowing?. Rust and copilot are helpful, telling me how to fix the problems, but I suspect there are many times I am making copies of objects when I could just be using references.

### Day 5

Well, that escalated quickly! Part 1 is simple. Part 2 shows that it wasn't a scalable solution. Manipulating intervals is interesting stuff. I learned there is something called an interval tree, but I don't think it has any use here.

### Day 6

Pretty trivial. I can imagine someone iterating through the possibilities, but the solution for both parts is to solve the quadratic equation and the answer is simply the difference between the roots.

### Day 7

Pretty trivial again. Lack of good test data made debugging difficult. Speaking of debugging, I can not figure out how to debug with command line arguments.
