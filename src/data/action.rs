use std::str;

#[derive(Debug, PartialEq)]
pub struct Action {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl str::FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() == 2 {
            let from = s[0..1].parse()?;
            let to = s[1..2].parse()?;

            if from == to {
                Err("The source and destination are the same.")
            } else if let Coordinate::Foundation(_) = from {
                Err("You cannot take a card from a foundation.")
            } else {
                Ok(Action { from, to })
            }
        } else {
            Err("Invalid input.")
        }
    }
}

#[cfg(test)]
mod test_action {
    use super::{Action, Coordinate};

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Action {
                from: Coordinate::Cascade(0),
                to: Coordinate::Cell(0)
            }),
            "1a\n".parse::<Action>(),
        );
    }

    #[test]
    fn invalid() {
        assert_eq!(Err("Invalid input."), "0a\n".parse::<Action>());
        assert_eq!(
            Err("The source and destination are the same."),
            "aa\n".parse::<Action>()
        );
        assert_eq!(Err("Invalid input."), "\n".parse::<Action>());
        assert_eq!(Err("Invalid input."), "1a1\n".parse::<Action>());
        assert_eq!(Err("Invalid input."), "".parse::<Action>());
    }
}

#[derive(Debug, PartialEq)]
pub enum Coordinate {
    Cascade(u8),
    Cell(u8),
    Foundation(u8),
}

impl str::FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().next() {
            match c {
                '1'..='8' => Ok(Coordinate::Cascade(c.to_digit(10).unwrap() as u8 - 1)),
                'a'..='d' | 'A'..='D' => Ok(Coordinate::Cell(c.to_digit(36).unwrap() as u8 - 10)),
                'w'..='z' | 'W'..='Z' => {
                    Ok(Coordinate::Foundation(c.to_digit(36).unwrap() as u8 - 32))
                }
                '!' => Ok(Coordinate::Cascade(0)),
                '@' => Ok(Coordinate::Cascade(1)),
                '#' => Ok(Coordinate::Cascade(2)),
                '$' => Ok(Coordinate::Cascade(3)),
                '%' => Ok(Coordinate::Cascade(4)),
                '^' => Ok(Coordinate::Cascade(5)),
                '&' => Ok(Coordinate::Cascade(6)),
                '*' => Ok(Coordinate::Cascade(7)),
                _ => Err("Invalid input."),
            }
        } else {
            Err("Invalid input.")
        }
    }
}

#[cfg(test)]
mod test_coordinate {
    use super::Coordinate;

    #[test]
    fn cascade() {
        assert_eq!(Ok(Coordinate::Cascade(0)), "1".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(1)), "2".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(2)), "3".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(3)), "4".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(4)), "5".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(5)), "6".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(6)), "7".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(7)), "8".parse::<Coordinate>());

        assert_eq!(Ok(Coordinate::Cascade(0)), "!".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(1)), "@".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(2)), "#".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(3)), "$".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(4)), "%".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(5)), "^".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(6)), "&".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cascade(7)), "*".parse::<Coordinate>());
    }

    #[test]
    fn cell() {
        assert_eq!(Ok(Coordinate::Cell(0)), "A".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cell(0)), "a".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cell(1)), "B".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cell(1)), "b".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cell(2)), "C".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cell(2)), "c".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cell(3)), "D".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Cell(3)), "d".parse::<Coordinate>());
    }

    #[test]
    fn foundation() {
        assert_eq!(Ok(Coordinate::Foundation(0)), "W".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Foundation(0)), "w".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Foundation(1)), "X".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Foundation(1)), "x".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Foundation(2)), "Y".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Foundation(2)), "y".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Foundation(3)), "Z".parse::<Coordinate>());
        assert_eq!(Ok(Coordinate::Foundation(3)), "z".parse::<Coordinate>());
    }

    #[test]
    fn invalid() {
        assert_eq!(Err("Invalid input."), "9".parse::<Coordinate>());
        assert_eq!(Err("Invalid input."), "0".parse::<Coordinate>());
        assert_eq!(Err("Invalid input."), "E".parse::<Coordinate>());
        assert_eq!(Err("Invalid input."), "}".parse::<Coordinate>());
        assert_eq!(Err("Invalid input."), "".parse::<Coordinate>());
    }
}
