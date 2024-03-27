use proptest::prelude::*;
use mockall::{automock, mock, predicate::*, Sequence};
use criterion::{criterion_group, criterion_main, Criterion};


#[cfg_attr(test, automock)]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}


proptest! {
    #[test]
    fn test_string_reversal(s: String) {
        let reversed = s.chars().rev().collect::<String>();
        assert_eq!(s, reversed.chars().rev().collect::<String>());
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_greeting() {
        let mut mock = MockGreeter::new();

        mock.expect_greet()
            .with(mockall::predicate::eq("Alice"))
            .times(1)
            .returning(|_| String::from("Hello, Alice"));
        assert_eq!("Hello, Alice", mock.greet("Alice"));
    }

    #[test]
    fn test_greeting_sequence() {
        let mut seq = Sequence::new();
        let mut mock = MockGreeter::new();
        mock.expect_greet()
            .with(mockall::predicate::eq("Alice"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| String::from("Hello, Alice"));
        mock.expect_greet()
            .with(mockall::predicate::eq("Bob"))
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| String::from("Hello, Bob"));
        assert_eq!("Hello, Alice", mock.greet("Alice"));
        assert_eq!("Hello, Bob", mock.greet("Bob"));
    }

}
