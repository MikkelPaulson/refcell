use crate::deck;
use crate::layout;
use crate::widget;
use druid::widget::prelude::*;
use druid::WidgetPod;

pub struct Cascade {
    column: u8,
    cards: Vec<druid::WidgetPod<layout::Tableau, widget::Card>>,
}

impl Cascade {
    pub fn new(column: u8) -> Self {
        Self {
            column,
            cards: Vec::new(),
        }
    }

    fn update_cards(&mut self, tableau: &layout::Tableau) {
        self.cards.clear();
        //if self.cards.is_empty() {
        let cascade: &layout::Cascade = &tableau.cascades[self.column as usize];
        let data_cards: &Vec<deck::Card> = cascade.cards();

        for data_card in data_cards.iter() {
            self.cards
                .push(WidgetPod::new(widget::Card::new(data_card)));
        }
        //}
    }
}

impl Widget<layout::Tableau> for Cascade {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut layout::Tableau, env: &Env) {
        self.update_cards(data);
        for child in &mut self.cards {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &layout::Tableau,
        env: &Env,
    ) {
        self.update_cards(data);
        for child in &mut self.cards {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &layout::Tableau,
        data: &layout::Tableau,
        env: &Env,
    ) {
        self.update_cards(data);
        for child in &mut self.cards {
            child.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &layout::Tableau,
        env: &Env,
    ) -> Size {
        let size = bc.max();
        for child in &mut self.cards {
            child.layout(ctx, bc, data, env);
        }
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &layout::Tableau, env: &Env) {
        for child in &mut self.cards {
            child.paint(ctx, data, env);
        }
    }
}
