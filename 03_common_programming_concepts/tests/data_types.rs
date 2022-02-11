extern crate core;

#[test]
fn calculate_float() {
    let x = 3 / 2;
    assert_eq!(x, 1);

    let x = 3.0 / 2.0;
    assert_eq!(x, 1.5);

    let x = 3.0 % 1.5;
    assert_eq!(x, 0.0);

    let x = 3.0 % 2.5;
    assert_eq!(x, 0.5);
}

#[test]
fn manipulate_tuple() {
    let t = (12, 12.34, "😺", false);

    let (_, float, _, boolean) = t;
    assert_eq!(float, 12.34);
    assert_eq!(boolean, false);
    assert_eq!(t.0, 12);
    assert_eq!(t.2, "😺");

    let mut t = t;
    t.1 = 23.45;

    let (int, float, _, _) = t;
    assert_eq!(int, 12);
    assert_eq!(float, 23.45);
}

#[test]
fn manipulate_array() {
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a[0], 1);

    let a = [4; 3];
    assert_eq!(a.len(), 3);
    assert_eq!(a, [4, 4, 4]);
}