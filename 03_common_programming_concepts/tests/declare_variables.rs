#[test]
fn single_variable() {
    let x = 4;
    assert_eq!(x, 4);
}

#[test]
fn mutable() {
    let mut x = 4;
    x = x + 4;
    assert_eq!(x, 8);
}

#[test]
fn shadowing() {
    let x = 4;
    let x = x + 4;
    assert_eq!(x, 8);
}

#[test]
fn block_scope() {
    let x = 4;
    {
        let x = x + 4;
        assert_eq!(x, 8);
    }
    assert_eq!(x, 4);
}
