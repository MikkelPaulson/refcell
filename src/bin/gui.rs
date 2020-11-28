use druid::widget as base_widget;
use druid::{AppLauncher, Color, Widget, WidgetExt, WindowDesc};

use refcell::data;
use refcell::widget;

fn main() {
    let tableau = data::Tableau::deal(data::Deck::shuffled());
    let main_window = WindowDesc::new(widget::Tableau::new);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(tableau)
        .expect("launch failed");
}
