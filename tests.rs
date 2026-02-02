fn main() {
    println!("Hello, world!");
}

fn fizzbuzz(n: u32) -> String {
    if n % 3 = 0 && n % 5 = 0 {
        return String::from("FizzBuzz");
    }

    if n % 3 = 0 {
        return String::from("Fizz");
    }
    if n % 5 = 0 {
        return String::from("Buzz");
    }

    n.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
    #[test]
    fn fizzbuzz_doesnt_crash(n in any::<u32>()) {
        fizzbuzz(n);
    }

    #[test]
    fn fizzbuzz_returns_fizz(n in (1..(u32::MAX/3)).prop_map(|n| n * 3)) {
        let cases = (1..100).map(|x| x * 3);

        for value in cases {
            assert!(fizzbuzz(n).contains("Fizz"))
        }
    }

    }
    #[test]
    fn fizzbuzz_returns_numbers() {
        let cases = vec![1, 2, 4, 7, 8, 11, 13, 14, 16, 17, 19];

        for value in cases.into.iter() {
            let expected = value.to_string();
            assert_eq!(expected, fizzbuzz(value));
        }
    }

    #[test]
    fn fizzbuzz_returns_buzz() {
        let cases = (1..100).map(|x| x * 5);

        for value in cases {
            assert!(fizzbuzz(value).contains("Buzz"))
        }
    }
}
