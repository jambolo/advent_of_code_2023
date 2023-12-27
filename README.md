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

Pretty trivial again. Lack of good test data made debugging difficult. Speaking of debugging, I cannot figure out how to debug with command line arguments.

### Day 8

Again pretty trivial, but part 2 involved some manual work in order to discover the cycles and determine their lengths, and then to deal with fact that they are not prime.

As for Rust, HashMap is convenient, but unlike C/C++, `String` is not `str` and not `Vec<char>`. 

Also, a function returning a reference requires specifying the lifetime of the reference, and it is not clear that you can't specify the wrong lifetime. For example, I returned a reference to a string in an element of a HashMap and specified (I believe) that the reference's lifetime is the same as the HashMap, but that is not correct, is it? Does the lifetime of a HashMap end when an element is removed from it, or does my specification ensure that the reference String is not destroyed until the HashMap is destroyed?

### Day 9

Again trivial. In Rust, you have to make a conscious effort to avoid making copies of objects.
