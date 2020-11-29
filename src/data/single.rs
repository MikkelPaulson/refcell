use super::Card;

pub trait Single {
    fn peek(&self) -> Option<&Card>;

    fn is_empty(&self) -> bool;
}
