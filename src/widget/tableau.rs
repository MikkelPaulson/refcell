use super::{Card, Single};
use crate::data;
use druid::lens;
use druid::widget::prelude::*;
use druid::widget::{Container, Flex, WidgetExt};
use druid::{Color, LensExt};

pub struct Tableau {
    child: Flex<data::Tableau>,
}

impl Tableau {
    pub fn new() -> Self {
        let mut row: Flex<data::Tableau> = Flex::row();
        for i in 0..8 {
            let mut column = Flex::column();

            if i < 4 {
                column.add_child(
                    Container::new(Single::new())
                        .background(Color::rgb8(63, 63, 63))
                        .padding(5.)
                        .lens(lens::Id.map(
                            move |t: &data::Tableau| t.cells[i].clone(),
                            move |t: &mut data::Tableau, c: data::Cell| t.cells[i] = c,
                        )),
                );
            } else {
                column.add_child(
                    Container::new(Card::new(&data::Card::new(i as u8 + 1, data::Suit::Hearts)))
                        .background(Color::rgb8(63, 63, 63))
                        .padding(5.)
                        .lens(lens::Id.map(|_| (), |_, _| ())),
                );
            }

            column.add_flex_child(
                Container::new(Card::new(&data::Card::new(i as u8 + 1, data::Suit::Clubs)))
                    .background(Color::rgb8(63, 63, 63))
                    .padding(5.)
                    .lens(lens::Id.map(|_| (), |_, _| ())),
                1.,
            );

            row.add_flex_child(column, 1.);
        }
        Tableau { child: row }
    }
}

impl Widget<data::Tableau> for Tableau {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut data::Tableau, env: &Env) {
        self.child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        lifecycle: &LifeCycle,
        data: &data::Tableau,
        env: &Env,
    ) {
        self.child.lifecycle(ctx, lifecycle, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &data::Tableau,
        data: &data::Tableau,
        env: &Env,
    ) {
        self.child.update(ctx, old_data, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &data::Tableau,
        env: &Env,
    ) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &data::Tableau, env: &Env) {
        self.child.paint(ctx, data, env)
    }
}
