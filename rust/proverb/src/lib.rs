pub fn build_proverb(list: &[&str]) -> String {
    const MAX_LINE: usize = 42;
    let mut result = String::with_capacity(list.len() * (MAX_LINE + 1));
    for pair in list.windows(2) {
        result.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            pair[0], pair[1]
        ));
    }
    if !list.is_empty() {
        result.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    result
}
