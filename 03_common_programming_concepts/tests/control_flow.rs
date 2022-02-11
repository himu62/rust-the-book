#[test]
fn if_else() {
    let tuples = [(1, "a"), (2, "b"), (3, "c")];
    for tuple in tuples {
        if tuple.0 == 1 {
            assert_eq!(tuple.1, "a");
        }
        else if tuple.0 == 2 {
            assert_eq!(tuple.1, "b");
        }
        else {
            assert_eq!(tuple.1, "c");
        }
    }

    let c = true;
    let n = if c {1} else {2};
    assert_eq!(n, 1);
}

#[test]
fn loops() {
    let mut counter = 1;
    let value = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2
        }
    };
    assert_eq!(value, 20);

    let mut number = 1;
    while number < 10 {
        number *= 2;
    }
    assert_eq!(number, 16);
}

#[test]
fn convert_temperature() {
    fn convert(input_number: f64, input_unit: &str) -> (f64, &str) {
        match input_unit {
            "Fahrenheit" => ((input_number - 32.0) / 1.8, "Celsius"),
            "Celsius" => (input_number * 1.8 + 32.0, "Fahrenheit"),
            _ => panic!("unit must be Fahrenheit or Celsius"),
        }
    }
    assert_eq!(convert(15.0, "Celsius"), (59.0, "Fahrenheit"));
    assert_eq!(convert(86.0, "Fahrenheit"), (30.0, "Celsius"));
}

#[test]
fn generate_fibonacci_number() {
    fn generate(nth: u32) -> u32 {
        if nth <= 2 {
            return nth;
        }
        let mut n = 1;
        let mut m = 1;
        let mut counter = 3;
        while counter < nth {
            counter += 1;
            let t = m;
            m = n + m;
            n = t;
        }
        return n + m;
    }
    assert_eq!(generate(0), 0);
    assert_eq!(generate(5), 5);
    assert_eq!(generate(9), 34);
    assert_eq!(generate(30), 832040);
}
