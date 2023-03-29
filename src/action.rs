use core::num::NonZeroU8;
use std::str;

#[derive(Debug, PartialEq)]
pub struct Action {
    pub from: FromCoordinate,
    pub to: ToCoordinate,
    pub count: Option<NonZeroU8>,
}

impl str::FromStr for Action {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (input_count, input_coords) =
            if let Some((input_count, input_coords)) = input.trim().split_once('+') {
                (Some(input_count), input_coords)
            } else {
                (None, input.trim())
            };

        let count = if let Some(input_count) = input_count {
            if input_count.is_empty() {
                Some(u8::MAX.try_into().unwrap())
            } else {
                Some(input_count.parse().map_err(|_| "Invalid count")?)
            }
        } else {
            None
        };

        if input_coords.len() == 2 {
            let from = input_coords[0..1].parse()?;
            let to = input_coords[1..2].parse()?;

            if from == to {
                Err("The source and destination are the same.")
            } else {
                Ok(Action { from, to, count })
            }
        } else {
            Err("Invalid input.")
        }
    }
}

#[cfg(test)]
mod test_action {
    use super::{Action, FromCoordinate, ToCoordinate};

    #[test]
    fn valid() {
        assert_eq!(
            Ok(Action {
                from: FromCoordinate::Cascade(0),
                to: ToCoordinate::Cell(0),
                count: None,
            }),
            "1a\n".parse::<Action>(),
        );

        assert_eq!(
            Ok(Action {
                from: FromCoordinate::Cascade(6),
                to: ToCoordinate::Cascade(7),
                count: Some(255.try_into().unwrap()),
            }),
            "+78\n".parse::<Action>(),
        );

        assert_eq!(
            Ok(Action {
                from: FromCoordinate::Cascade(6),
                to: ToCoordinate::Cascade(7),
                count: Some(15.try_into().unwrap()),
            }),
            "15+78\n".parse::<Action>(),
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
pub enum FromCoordinate {
    Cascade(u8),
    Cell(u8),
}

#[derive(Debug, PartialEq)]
pub enum ToCoordinate {
    Cascade(u8),
    Cell(u8),
    Foundation(u8),
}

impl str::FromStr for FromCoordinate {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.parse::<ToCoordinate>() {
            Ok(ToCoordinate::Cascade(n)) => Ok(FromCoordinate::Cascade(n)),
            Ok(ToCoordinate::Cell(n)) => Ok(FromCoordinate::Cell(n)),
            Ok(ToCoordinate::Foundation(_)) => Err("You cannot take a card from a foundation."),
            Err(e) => Err(e),
        }
    }
}

impl str::FromStr for ToCoordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().next() {
            match c {
                '1'..='8' => Ok(ToCoordinate::Cascade(c.to_digit(10).unwrap() as u8 - 1)),
                'a'..='d' | 'A'..='D' => Ok(ToCoordinate::Cell(c.to_digit(36).unwrap() as u8 - 10)),
                'w'..='z' | 'W'..='Z' => {
                    Ok(ToCoordinate::Foundation(c.to_digit(36).unwrap() as u8 - 32))
                }
                '!' => Ok(ToCoordinate::Cascade(0)),
                '@' => Ok(ToCoordinate::Cascade(1)),
                '#' => Ok(ToCoordinate::Cascade(2)),
                '$' => Ok(ToCoordinate::Cascade(3)),
                '%' => Ok(ToCoordinate::Cascade(4)),
                '^' => Ok(ToCoordinate::Cascade(5)),
                '&' => Ok(ToCoordinate::Cascade(6)),
                '*' => Ok(ToCoordinate::Cascade(7)),
                _ => Err("Invalid input."),
            }
        } else {
            Err("Invalid input.")
        }
    }
}

impl PartialEq<ToCoordinate> for FromCoordinate {
    fn eq(&self, other: &ToCoordinate) -> bool {
        match (self, other) {
            (FromCoordinate::Cascade(a), ToCoordinate::Cascade(b)) => a == b,
            (FromCoordinate::Cell(a), ToCoordinate::Cell(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq<FromCoordinate> for ToCoordinate {
    fn eq(&self, other: &FromCoordinate) -> bool {
        other == self
    }
}

#[cfg(test)]
mod test_coordinate {
    use super::{FromCoordinate, ToCoordinate};

    #[test]
    fn cascade_from() {
        assert_eq!(
            Ok(FromCoordinate::Cascade(0)),
            "1".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(1)),
            "2".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(2)),
            "3".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(3)),
            "4".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(4)),
            "5".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(5)),
            "6".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(6)),
            "7".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(7)),
            "8".parse::<FromCoordinate>(),
        );

        assert_eq!(
            Ok(FromCoordinate::Cascade(0)),
            "!".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(1)),
            "@".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(2)),
            "#".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(3)),
            "$".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(4)),
            "%".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(5)),
            "^".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(6)),
            "&".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Ok(FromCoordinate::Cascade(7)),
            "*".parse::<FromCoordinate>(),
        );
    }

    #[test]
    fn cascade_to() {
        assert_eq!(Ok(ToCoordinate::Cascade(0)), "1".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(1)), "2".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(2)), "3".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(3)), "4".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(4)), "5".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(5)), "6".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(6)), "7".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(7)), "8".parse::<ToCoordinate>());

        assert_eq!(Ok(ToCoordinate::Cascade(0)), "!".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(1)), "@".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(2)), "#".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(3)), "$".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(4)), "%".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(5)), "^".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(6)), "&".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cascade(7)), "*".parse::<ToCoordinate>());
    }

    #[test]
    fn cell_from() {
        assert_eq!(Ok(FromCoordinate::Cell(0)), "A".parse::<FromCoordinate>());
        assert_eq!(Ok(FromCoordinate::Cell(0)), "a".parse::<FromCoordinate>());
        assert_eq!(Ok(FromCoordinate::Cell(1)), "B".parse::<FromCoordinate>());
        assert_eq!(Ok(FromCoordinate::Cell(1)), "b".parse::<FromCoordinate>());
        assert_eq!(Ok(FromCoordinate::Cell(2)), "C".parse::<FromCoordinate>());
        assert_eq!(Ok(FromCoordinate::Cell(2)), "c".parse::<FromCoordinate>());
        assert_eq!(Ok(FromCoordinate::Cell(3)), "D".parse::<FromCoordinate>());
        assert_eq!(Ok(FromCoordinate::Cell(3)), "d".parse::<FromCoordinate>());
    }

    #[test]
    fn cell_to() {
        assert_eq!(Ok(ToCoordinate::Cell(0)), "A".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cell(0)), "a".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cell(1)), "B".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cell(1)), "b".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cell(2)), "C".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cell(2)), "c".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cell(3)), "D".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Cell(3)), "d".parse::<ToCoordinate>());
    }

    #[test]
    fn foundation_from() {
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "W".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "w".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "X".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "x".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "Y".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "y".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "Z".parse::<FromCoordinate>(),
        );
        assert_eq!(
            Err("You cannot take a card from a foundation."),
            "z".parse::<FromCoordinate>(),
        );
    }

    #[test]
    fn foundation_to() {
        assert_eq!(Ok(ToCoordinate::Foundation(0)), "W".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Foundation(0)), "w".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Foundation(1)), "X".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Foundation(1)), "x".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Foundation(2)), "Y".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Foundation(2)), "y".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Foundation(3)), "Z".parse::<ToCoordinate>());
        assert_eq!(Ok(ToCoordinate::Foundation(3)), "z".parse::<ToCoordinate>());
    }

    #[test]
    fn invalid() {
        assert_eq!(Err("Invalid input."), "9".parse::<FromCoordinate>());
        assert_eq!(Err("Invalid input."), "0".parse::<FromCoordinate>());
        assert_eq!(Err("Invalid input."), "E".parse::<FromCoordinate>());
        assert_eq!(Err("Invalid input."), "}".parse::<FromCoordinate>());
        assert_eq!(Err("Invalid input."), "".parse::<FromCoordinate>());

        assert_eq!(Err("Invalid input."), "9".parse::<ToCoordinate>());
        assert_eq!(Err("Invalid input."), "0".parse::<ToCoordinate>());
        assert_eq!(Err("Invalid input."), "E".parse::<ToCoordinate>());
        assert_eq!(Err("Invalid input."), "}".parse::<ToCoordinate>());
        assert_eq!(Err("Invalid input."), "".parse::<ToCoordinate>());
    }

    #[test]
    fn eq() {
        assert_eq!(FromCoordinate::Cascade(0), ToCoordinate::Cascade(0));
        assert_eq!(ToCoordinate::Cascade(0), FromCoordinate::Cascade(0));
        assert_eq!(FromCoordinate::Cell(0), ToCoordinate::Cell(0));
        assert_eq!(ToCoordinate::Cell(0), FromCoordinate::Cell(0));

        assert_ne!(FromCoordinate::Cascade(0), ToCoordinate::Cascade(1));
        assert_ne!(ToCoordinate::Cascade(0), FromCoordinate::Cascade(1));
        assert_ne!(FromCoordinate::Cell(0), ToCoordinate::Cell(1));
        assert_ne!(ToCoordinate::Cell(0), FromCoordinate::Cell(1));
        assert_ne!(FromCoordinate::Cascade(0), ToCoordinate::Cell(0));
        assert_ne!(ToCoordinate::Cell(0), FromCoordinate::Cascade(0));
        assert_ne!(FromCoordinate::Cascade(0), ToCoordinate::Foundation(0));
        assert_ne!(FromCoordinate::Cell(0), ToCoordinate::Foundation(0));
    }
}
