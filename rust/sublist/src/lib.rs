use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let (m, n) = (first_list.len(), second_list.len());

    match m.cmp(&n) {
        Ordering::Less => {
            if first_list.is_empty() || second_list
                .windows(first_list.len())
                .any(|x| x == first_list) {
                    return Comparison::Sublist;
                }
        }
        Ordering::Greater => {
            if second_list.is_empty() || first_list
                .windows(second_list.len())
                .any(|x| x == second_list) {
                    return Comparison::Superlist;
                }
        }
        Ordering::Equal => {
            if first_list == second_list {
                return Comparison::Equal;
            }
        }
    }

    Comparison::Unequal
}
