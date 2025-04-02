pub fn to_int(s: String) -> i32 {
    s.parse().expect("Failed to parse number")
}