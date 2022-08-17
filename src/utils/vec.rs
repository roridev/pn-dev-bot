#[inline(always)]
pub fn stringify_vec(vec: &Vec<String>, if_empty: &str, join: &str) -> String {
    if vec.is_empty() {
        if_empty.to_string()
    } else {
        vec.join(join)
    }
}
