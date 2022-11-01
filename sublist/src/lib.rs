#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len();
    let second_len = _second_list.len();
    if first_len == second_len {
        if first_len == 0 {
            return Comparison::Equal;
        }
        if second_len == 0 {
            return Comparison::Equal;
        }
    }
    if first_len > second_len {
        if second_len == 0 {
            return Comparison::Superlist;
        }
    }
    if first_len < second_len {
        if first_len == 0 {
            return Comparison::Sublist;
        }
    }
    Comparison::Unequal
}
