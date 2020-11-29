use super::Card;
use crate::data;
use druid::widget::prelude::*;

pub struct Cell {
    card: Option<Card>,
}

impl Cell {
    pub fn new() -> Self {
        Self { card: None }
    }
}

impl Widget<data::Cell> for Cell {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut data::Cell, env: &Env) {
        self.card
            .as_mut()
            .map(|card| card.event(ctx, event, &mut (), env));
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        lifecycle: &LifeCycle,
        _data: &data::Cell,
        env: &Env,
    ) {
        self.card
            .as_mut()
            .map(|card| card.lifecycle(ctx, lifecycle, &mut (), env));
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &data::Cell, data: &data::Cell, env: &Env) {
        match (old_data.is_empty(), data.is_empty()) {
            (true, false) => self.card = data.peek().map(|card| Card::new(card)),
            (false, true) => self.card = None,
            _ => {}
        }

        self.card
            .as_mut()
            .map(|card| card.update(ctx, &mut (), &mut (), env));
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &data::Cell,
        env: &Env,
    ) -> Size {
        self.card
            .as_mut()
            .map(|card| card.layout(ctx, bc, &(), env))
            .unwrap_or_else(|| Card::get_size(bc))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &data::Cell, env: &Env) {
        self.card.as_mut().map(|card| card.paint(ctx, &(), env));
    }
}
