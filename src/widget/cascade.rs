use super::Card;
use crate::data;
use druid::widget::prelude::*;
use druid::{Point, Rect, WidgetPod};
use std::cmp;

pub struct Cascade {
    cards: Vec<WidgetPod<(), Card>>,
}

impl Cascade {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn update_cards(&mut self, data: &data::Cascade) {
        let widget_len = self.cards.len();
        let data_len = data.cards().len();

        match widget_len.cmp(&data_len) {
            cmp::Ordering::Less => {
                for i in widget_len..data_len {
                    self.cards.push(WidgetPod::new(Card::new(&data.cards()[i])));
                }
            }
            cmp::Ordering::Greater => {
                self.cards.truncate(data_len);
            }
            cmp::Ordering::Equal => {}
        }
    }
}

impl Widget<data::Cascade> for Cascade {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut data::Cascade, env: &Env) {
        self.update_cards(data);
        for child in &mut self.cards {
            child.event(ctx, event, &mut (), env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &data::Cascade,
        env: &Env,
    ) {
        self.update_cards(data);
        for child in &mut self.cards {
            child.lifecycle(ctx, event, &(), env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &data::Cascade,
        data: &data::Cascade,
        env: &Env,
    ) {
        self.update_cards(data);
        for child in &mut self.cards {
            child.update(ctx, &(), env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &data::Cascade,
        env: &Env,
    ) -> Size {
        let container_size = bc.max();

        match self.cards.len() {
            0 => {}
            1 => self.cards[0].set_layout_rect(
                ctx,
                &(),
                env,
                Rect::from_origin_size(Point::ORIGIN, Card::get_size(bc)),
            ),
            _ => {
                let card_size = Card::get_size(bc);
                let step_height = ((container_size.height - card_size.height)
                    / (self.cards.len() - 1) as f64)
                    .min(card_size.height / 5.);

                for i in 0..self.cards.len() {
                    self.cards[i].set_layout_rect(
                        ctx,
                        &(),
                        env,
                        Rect::from_origin_size(
                            Point::new(0., step_height * i as f64),
                            Card::get_size(bc),
                        ),
                    );
                }
            }
        }

        container_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &data::Cascade, env: &Env) {
        for child in &mut self.cards {
            child.paint(ctx, &(), env);
        }
    }
}
