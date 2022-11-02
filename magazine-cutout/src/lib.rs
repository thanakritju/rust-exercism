// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazineMap: HashMap<&str, i32> = HashMap::new();
    for each in magazine {
        magazineMap.entry(each).and_modify(|e| *e += 1).or_insert(1);
    }
    for each in note {
        magazineMap
            .entry(each)
            .and_modify(|e| *e -= 1)
            .or_insert(-1);
    }
    !magazineMap.into_iter().any(|(key, value)| value <= -1)
}
