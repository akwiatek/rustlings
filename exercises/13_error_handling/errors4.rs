#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value.cmp(&0) {
            std::cmp::Ordering::Greater => Ok(Self(value as u64)),
            std::cmp::Ordering::Equal => Err(CreationError::Zero),
            std::cmp::Ordering::Less => Err(CreationError::Negative),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_1() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10))
        );
    }

    #[test]
    fn test_creation_2() {
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
    }

    #[test]
    fn test_creation_3() {
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
