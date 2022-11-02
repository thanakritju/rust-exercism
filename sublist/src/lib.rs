use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Display>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    let mut map = HashMap::new();
    for each in _first_list {
        let key = format!("{}", each);
        map.entry(key).and_modify(|e| *e += 1).or_insert(1);
    }
    for each in _second_list {
        let key = format!("{}", each);
        map.entry(key).and_modify(|e| *e -= 1).or_insert(-1);
    }

    if map.values().into_iter().all(|&value| value == 0) {
        return Comparison::Equal;
    }

    if map.values().into_iter().all(|&value| value >= 0) {
        return Comparison::Superlist;
    }

    if map.values().into_iter().all(|&value| value <= 0) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
