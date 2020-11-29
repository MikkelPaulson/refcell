use super::Card;
use crate::data;
use crate::widget;
use druid::widget::prelude::*;

pub struct Cascade {
    cards: Vec<widget::Card>,
}

impl Cascade {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn update_cards(&mut self, data: &data::Cascade) {
        let widget_len = self.cards.len();
        let data_len = data.cards().len();

        if widget_len > data_len {
            self.cards.truncate(data_len);
        } else if widget_len < data_len {
            for i in widget_len..data_len {
                self.cards.push(Card::new(&data.cards()[i]));
            }
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
            child.update(ctx, &(), &(), env);
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &data::Cascade,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &data::Cascade, env: &Env) {
        for child in &mut self.cards {
            child.paint(ctx, &(), env);
        }
    }
}
