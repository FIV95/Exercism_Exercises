extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut result = String::new(); // a string to hold the final result we will return
    // going to create an iterator object that will go over the input string
    let iterator = UnicodeSegmentation::graphemes(input, true)
    .rev(); // reverse the order of the strings
    .collect(); // collect the iterator into a vector
    stack.push(iterator);


}
