use std::fmt;

pub fn fmissing(expected: impl fmt::Display) -> String {
    format!("Missing '{}'", expected)
}

pub fn finvalid(expected: impl fmt::Display, actual: String) -> String {
    format!("Invalid {}, found '{}'", expected, actual)
}

pub fn finvalidkind(
    kind: impl fmt::Display,
    expected: impl fmt::Display,
    actual: String,
) -> String {
    format!(
        "Invalid {}, expected {}, found '{}'",
        kind, expected, actual
    )
}
