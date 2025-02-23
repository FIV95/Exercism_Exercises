#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {

    if first_list == second_list {
        return Comparison::Equal; 
    }

    if first_list.is_empty() {
        return Comparison::Sublist;
    }
    
    if second_list.is_empty() {
        return Comparison::Superlist;
    }


    for window in second_list.windows(first_list.len()) {
        if window == first_list {
            return Comparison::Sublist;
        }
    }
    for window in first_list.windows(second_list.len()) {
        if window == second_list {
            return Comparison::Superlist;
        }
    }

    return Comparison::Unequal;


    }


