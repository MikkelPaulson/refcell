use druid::widget::{Container, Flex, Label};
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

mod deck;

fn main() {
    let main_window = WindowDesc::new(ui_builder);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<()> {
    let mut row = Flex::row();

    for i in 0..=7 {
        row.add_flex_child(
            Flex::column()
                .with_child(
                    Container::new(Label::new(i.to_string()).padding(5.0).center())
                        .background(Color::rgb8(63, 63, 63))
                        .padding(5.0),
                )
                .with_flex_child(
                    Container::new(Label::new(i.to_string()).padding(5.0).center())
                        .background(Color::rgb8(63, 63, 63))
                        .padding(5.0),
                    1.0,
                ),
            1.0,
        );
    }
    row
}
