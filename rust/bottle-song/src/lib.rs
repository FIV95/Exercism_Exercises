pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    let mut current_bottles = start_bottles;
    let mut take_down_count = take_down;
    let mut loop_count = 0;
    while take_down_count > 0 {

        let mut current_bottle_string = num_to_string(current_bottles);
        current_bottle_string = capitalize_first(&current_bottle_string);
    for _ in 0..2 {
        result += &format!("{current_bottle_string} green bottle{} hanging on the wall,\n",
            if current_bottles == 1 {""} else {"s"});
    }
    current_bottles -= 1;
    take_down_count -= 1;
    result += &format!("And if one green bottle should accidentally fall,\n");
    let current_bottles_string =
        match current_bottles {
            0 => "no".to_string(),
            2..=u32::MAX => num_to_string(current_bottles),
            1 => "one".to_string(),
        };

    result += &format!("There'll be {current_bottles_string} green bottle{} hanging on the wall.\n\n",
        if current_bottles == 1 {""} else {"s"});
    loop_count += 1;
    println!("{:?}", loop_count);
}
println!("{}", result);

    result.to_string()

}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(), // If the string is empty, return empty
    }
}

fn num_to_string (n : u32) -> String {
    let result: String = 
    match n {
        0 => String::from("no"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        _ => unreachable!(),
    };
    return result;
}