pub fn square(s: u32) -> u128 {
    if s <= 64 {
    return 2_u128.pow(s-1) as u128;
    }
    panic!()
}

pub fn total() -> u128 {
    (1..=64).map(square).sum()
}

/*
 * ## Instructions

Calculate the number of grains of wheat on a chessboard.

A chessboard has 64 squares.
Square 1 has one grain, square 2 has two grains, square 3 has four grains, and so on, doubling each time.

Write code that calculates:

- the number of grains on a given square
- the total number of grains on the chessboard

// 64 boards
// maximum number of grains:
// Square || Grain
//  1           1
//  2           2
//  3           4
//  4           8
//  5           16
//  6           32
//  7           64
//  8           128
//  9           256
//  10          512
//
//  so with 64 max tiles we can have a max number of grains: 9_223_372_036_854_775_808
//  the formula is grains = 2^n-1
//
*/
