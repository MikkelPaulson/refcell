use crate::data;
use druid::widget::prelude::*;
use druid::widget::Svg;

pub struct Card {
    svg: Svg,
}

impl Card {
    const WIDTH: f64 = 360.0;
    const HEIGHT: f64 = 540.0;

    pub fn new(card: &data::Card) -> Self {
        Self {
            svg: Svg::new(Self::get_svg(card).parse().unwrap()),
        }
    }

    fn get_svg(card: &data::Card) -> &'static str {
        match (card.get_rank(), card.get_suit()) {
            (1, data::Suit::Clubs) => include_str!("../../assets/cards/1C.svg"),
            (1, data::Suit::Diamonds) => include_str!("../../assets/cards/1D.svg"),
            (1, data::Suit::Hearts) => include_str!("../../assets/cards/1H.svg"),
            (1, data::Suit::Spades) => include_str!("../../assets/cards/1S.svg"),
            (2, data::Suit::Clubs) => include_str!("../../assets/cards/2C.svg"),
            (2, data::Suit::Diamonds) => include_str!("../../assets/cards/2D.svg"),
            (2, data::Suit::Hearts) => include_str!("../../assets/cards/2H.svg"),
            (2, data::Suit::Spades) => include_str!("../../assets/cards/2S.svg"),
            (3, data::Suit::Clubs) => include_str!("../../assets/cards/3C.svg"),
            (3, data::Suit::Diamonds) => include_str!("../../assets/cards/3D.svg"),
            (3, data::Suit::Hearts) => include_str!("../../assets/cards/3H.svg"),
            (3, data::Suit::Spades) => include_str!("../../assets/cards/3S.svg"),
            (4, data::Suit::Clubs) => include_str!("../../assets/cards/4C.svg"),
            (4, data::Suit::Diamonds) => include_str!("../../assets/cards/4D.svg"),
            (4, data::Suit::Hearts) => include_str!("../../assets/cards/4H.svg"),
            (4, data::Suit::Spades) => include_str!("../../assets/cards/4S.svg"),
            (5, data::Suit::Clubs) => include_str!("../../assets/cards/5C.svg"),
            (5, data::Suit::Diamonds) => include_str!("../../assets/cards/5D.svg"),
            (5, data::Suit::Hearts) => include_str!("../../assets/cards/5H.svg"),
            (5, data::Suit::Spades) => include_str!("../../assets/cards/5S.svg"),
            (6, data::Suit::Clubs) => include_str!("../../assets/cards/6C.svg"),
            (6, data::Suit::Diamonds) => include_str!("../../assets/cards/6D.svg"),
            (6, data::Suit::Hearts) => include_str!("../../assets/cards/6H.svg"),
            (6, data::Suit::Spades) => include_str!("../../assets/cards/6S.svg"),
            (7, data::Suit::Clubs) => include_str!("../../assets/cards/7C.svg"),
            (7, data::Suit::Diamonds) => include_str!("../../assets/cards/7D.svg"),
            (7, data::Suit::Hearts) => include_str!("../../assets/cards/7H.svg"),
            (7, data::Suit::Spades) => include_str!("../../assets/cards/7S.svg"),
            (8, data::Suit::Clubs) => include_str!("../../assets/cards/8C.svg"),
            (8, data::Suit::Diamonds) => include_str!("../../assets/cards/8D.svg"),
            (8, data::Suit::Hearts) => include_str!("../../assets/cards/8H.svg"),
            (8, data::Suit::Spades) => include_str!("../../assets/cards/8S.svg"),
            (9, data::Suit::Clubs) => include_str!("../../assets/cards/9C.svg"),
            (9, data::Suit::Diamonds) => include_str!("../../assets/cards/9D.svg"),
            (9, data::Suit::Hearts) => include_str!("../../assets/cards/9H.svg"),
            (9, data::Suit::Spades) => include_str!("../../assets/cards/9S.svg"),
            (10, data::Suit::Clubs) => include_str!("../../assets/cards/10C.svg"),
            (10, data::Suit::Diamonds) => include_str!("../../assets/cards/10D.svg"),
            (10, data::Suit::Hearts) => include_str!("../../assets/cards/10H.svg"),
            (10, data::Suit::Spades) => include_str!("../../assets/cards/10S.svg"),
            (11, data::Suit::Clubs) => include_str!("../../assets/cards/11C.svg"),
            (11, data::Suit::Diamonds) => include_str!("../../assets/cards/11D.svg"),
            (11, data::Suit::Hearts) => include_str!("../../assets/cards/11H.svg"),
            (11, data::Suit::Spades) => include_str!("../../assets/cards/11S.svg"),
            (12, data::Suit::Clubs) => include_str!("../../assets/cards/12C.svg"),
            (12, data::Suit::Diamonds) => include_str!("../../assets/cards/12D.svg"),
            (12, data::Suit::Hearts) => include_str!("../../assets/cards/12H.svg"),
            (12, data::Suit::Spades) => include_str!("../../assets/cards/12S.svg"),
            (13, data::Suit::Clubs) => include_str!("../../assets/cards/13C.svg"),
            (13, data::Suit::Diamonds) => include_str!("../../assets/cards/13D.svg"),
            (13, data::Suit::Hearts) => include_str!("../../assets/cards/13H.svg"),
            (13, data::Suit::Spades) => include_str!("../../assets/cards/13S.svg"),
            _ => unreachable!(),
        }
    }
}

impl Widget<()> for Card {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (), env: &Env) {
        self.svg.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(), env: &Env) {
        self.svg.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &(), data: &(), env: &Env) {
        self.svg.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(), env: &Env) -> Size {
        let mut size = bc.max();
        match (bc.is_width_bounded(), bc.is_height_bounded()) {
            (true, true) => {
                if (size.height / size.width) > (Self::HEIGHT / Self::WIDTH) {
                    size.height = size.width / Self::WIDTH * Self::HEIGHT;
                } else {
                    size.width = size.height / Self::HEIGHT * Self::WIDTH;
                }
            }
            (true, false) => {
                size.height = size.width / Self::WIDTH * Self::HEIGHT;
            }
            (false, true) => {
                size.width = size.height / Self::HEIGHT * Self::WIDTH;
            }
            (false, false) => {
                size = Size::new(Self::WIDTH, Self::HEIGHT);
            }
        }
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(), env: &Env) {
        self.svg.paint(ctx, data, env)
    }
}
