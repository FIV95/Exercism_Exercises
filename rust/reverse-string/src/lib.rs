extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // lets build a stack
    // does rust have a built in stack?
    let mut stack = Vec::new();
    // for each c in the input we add it to the stack
    for c in input.chars() {
        stack.push(c);
    }
    // okay so each character is now on the stack
    // we can create a new string - it can be empty for now
    let mut result = String::new();

    // we wanna keep popping the last element off the stack until the stack is empty
    while let Some(c) = stack.pop() { // c is gonna be the character popped
        result.push(c); // we add the character to the result string}

    }
    return result;
}
