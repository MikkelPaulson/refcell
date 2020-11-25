use druid::widget::{Container, Flex};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

mod deck;
mod layout;
mod widget;

use deck::Deck;
use layout::Tableau;

fn main() {
    let tableau = Tableau::deal(Deck::shuffled());
    let main_window = WindowDesc::new(ui_builder);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(tableau)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<Tableau> {
    let mut row = Flex::row();

    for _ in 0..8 {
        row.add_flex_child(
            Flex::column()
                .with_child(
                    Container::new(widget::Card::new(&deck::Card::new(1, deck::Suit::Hearts)))
                        .background(Color::rgb8(63, 63, 63))
                        .padding(5.0),
                )
                .with_flex_child(
                    Container::new(widget::Card::new(&deck::Card::new(1, deck::Suit::Clubs)))
                        .background(Color::rgb8(63, 63, 63))
                        .padding(5.0),
                    1.0,
                ),
            1.0,
        );
    }
    row
}
