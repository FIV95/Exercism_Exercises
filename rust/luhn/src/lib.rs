/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    
    // check length 1 or less: false
    // else
    // trim
    // allocate sum variable
    // start loop from back - every odd usize calculate
    // str[usize] * 2 GREATER THAN 9? Subtract 9 += sum
    // Else add str[usize]
    // if sum % 10 == 0 ? return true
    let trimmed = code.replace(" ", "");

    if trimmed.len() <= 1 {
        return false;
    } else {
        let mut sum = 0;
        println!("\n\n");
        for (index, value) in trimmed.chars().rev().enumerate() {
            print!("{:?} VALUE: {:?}\n", index, value);
            match value.to_digit(10) {
                Some(digit) => {
                    if index % 2 != 0 {
                        println!("DOUBLED");
                        let mut product = digit * 2;
                        if product > 9 {
                            product -= 9;
                            println!("SUBTRACTED 9 {:?}", product);
                            sum += product;
                            println!("Sum after adding subtracted product that was doubled {:?}", sum);
                        }
                        else {
                        dbg!(product, sum);
                        sum += product;
                        }
                    }
                    else{
                    sum += digit;
                    dbg!(sum);
                    }
                }
            None => return false,
                }
            }
            
            if sum % 10 == 0 {
                return true;
            }
            
    }
    return false;
    }