use super::Card;

pub trait Single: Sized {
    fn peek(&self) -> Option<&Card>;

    fn is_empty(&self) -> bool;
}
