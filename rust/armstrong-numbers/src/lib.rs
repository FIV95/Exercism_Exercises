pub fn is_armstrong_number(num: u32) -> bool {
    let length=  num.checked_ilog10().unwrap_or(0) + 1;
    let mut current_number = num;
    let mut result = 0;
    println!("num {:?}", num);

    while current_number > 0 {
        let digit = current_number % 10;
        println!("current_number {:?}", current_number);
        let number_to_be_added = digit.pow(length);
        result += number_to_be_added;
        println!("Result: {:?}", result);
        current_number /= 10
    }
    println!("Final Result {:?}", result);
    if result == num {
        return true
    }
    else {
    return false
    }
}
