pub(crate) fn split_first_char(string: &str) -> (char, String) {
    let mut chars = string.chars();

    let prefix = chars.next().expect("string was empty");
    let remaining = String::from_iter(chars);

    (prefix, remaining)
}
