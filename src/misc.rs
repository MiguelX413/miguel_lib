pub(crate) fn a_else_b<T>(condition: bool, a: T, b: T) -> T {
    if condition {
        a
    } else {
        b
    }
}
