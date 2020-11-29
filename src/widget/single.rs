use super::Card;
use crate::data;
use druid::widget::prelude::*;
use druid::Data;
use std::marker;

pub struct Single<T> {
    card: Option<Card>,
    phantom: marker::PhantomData<T>,
}

impl<T: data::Single + Data> Single<T> {
    pub fn new() -> Self {
        Self {
            card: None,
            phantom: marker::PhantomData,
        }
    }

    fn update_data(&mut self, data: &T) {
        match (self.card.is_none(), data.is_empty()) {
            (true, false) => self.card = data.peek().map(|card| Card::new(card)),
            (false, true) => self.card = None,
            _ => {}
        }
    }
}

impl<T: data::Single + Data> Widget<T> for Single<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, env: &Env) {
        self.card
            .as_mut()
            .map(|card| card.event(ctx, event, &mut (), env));
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, lifecycle: &LifeCycle, data: &T, env: &Env) {
        self.update_data(data);

        self.card
            .as_mut()
            .map(|card| card.lifecycle(ctx, lifecycle, &mut (), env));
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.update_data(data);

        self.card
            .as_mut()
            .map(|card| card.update(ctx, &mut (), &mut (), env));
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        self.card
            .as_mut()
            .map(|card| card.layout(ctx, bc, &(), env))
            .unwrap_or_else(|| Card::get_size(bc))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        self.card.as_mut().map(|card| card.paint(ctx, &(), env));
    }
}
