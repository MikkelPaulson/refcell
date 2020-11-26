use druid::widget::{Container, Flex};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

use refcell::data;
use refcell::widget;

fn main() {
    let tableau = data::Tableau::deal(data::Deck::shuffled());
    let main_window = WindowDesc::new(ui_builder);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(tableau)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<data::Tableau> {
    let mut row = Flex::row();

    for i in 0..8 {
        row.add_flex_child(
            Flex::column()
                .with_child(
                    Container::new(widget::Card::new(&data::Card::new(
                        i + 1,
                        data::Suit::Hearts,
                    )))
                    .background(Color::rgb8(63, 63, 63))
                    .padding(5.0),
                )
                .with_flex_child(
                    Container::new(widget::Cascade::new(i))
                        .background(Color::rgb8(63, 63, 63))
                        .padding(5.0),
                    1.0,
                ),
            1.0,
        );
    }
    row
}
