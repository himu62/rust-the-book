#[test]
fn expressions() {
    let x = {
        let y = 6;
        y * 2
    };
    assert_eq!(x, 12);

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    assert_eq!(add(2, 3), 5);
}