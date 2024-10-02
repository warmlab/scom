use gtk::{prelude::*, *};
use crate::panel::PanelTrait;

pub struct InputPanel;

impl InputPanel {
    fn build_control() -> Widget {
        let vbox = Box::new(Orientation::Vertical, 10);
        let button = Button::builder()
            .label("Clear")
            .build();

        let hbox = Box::new(Orientation::Horizontal, 10);
        let switch = Switch::new();
        switch.set_active(false);
        let label = Label::new(Some("Send as HEX code"));
        hbox.append(&switch);
        hbox.append(&label);

        vbox.append(&hbox);
        vbox.append(&button);

        vbox.into()
    }
}

impl PanelTrait for InputPanel {
    fn new() -> Self {
        InputPanel {}
    }

    fn build_panel(&self) -> Widget {
        let text_view = TextView::builder().editable(true).cursor_visible(true).overwrite(false).hexpand(true).vexpand(true).build();
        let buffer = text_view.buffer(); //.expect("Failed to get buffer");
        let mut iter = buffer.iter_at_line(0).unwrap();
        buffer.insert(&mut iter, "text");
        let (start, end) = buffer.bounds();
        println!("text view iter=[{:?}], start=[{:?}], end=[{:?}]", iter, start, end);

        let vbox = Box::new(Orientation::Vertical, 10);
        let control_box = InputPanel::build_control();
        vbox.append(&control_box);

        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.append(&vbox);
        hbox.append(&text_view);

        hbox.into()
    }
}
