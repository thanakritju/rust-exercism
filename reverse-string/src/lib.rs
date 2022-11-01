use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::from("");
    for c in input.graphemes(true) {
        reversed = format!("{}{}", c, reversed);
    }
    reversed
}
