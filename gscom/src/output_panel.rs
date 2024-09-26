use gtk::{prelude::*, *};

use crate::panel::PanelTrait;
use crate::port_panel::PortPanel;

pub struct OutputPanel;

impl OutputPanel {
    fn build_control() -> Widget {
        let vbox = Box::new(Orientation::Vertical, 10);
        let button = Button::builder()
            .label("Clear")
            .build();

        let hbox = Box::new(Orientation::Horizontal, 10);
        let switch = Switch::new();
        switch.set_active(false);
        let label = Label::new(Some("HEX code"));
        hbox.append(&switch);
        hbox.append(&label);

        vbox.append(&button);
        vbox.append(&hbox);

        vbox.into()
    }
}

impl PanelTrait for OutputPanel {
    fn build_panel() -> Widget {
        let text_view = TextView::builder().editable(false).cursor_visible(false).overwrite(true).hexpand(true).vexpand(true).build();
        let buffer = text_view.buffer(); //.expect("Failed to get buffer");
        let mut iter = buffer.iter_at_line(0).unwrap();
        buffer.insert(&mut iter, "text");
        let (start, end) = buffer.bounds();
        println!("text view iter=[{:?}], start=[{:?}], end=[{:?}]", iter, start, end);

        let vbox = Box::new(Orientation::Vertical, 10);
        let port_box = PortPanel::build_panel();
        let control_box = OutputPanel::build_control();
        vbox.append(&port_box);
        vbox.append(&control_box);

        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.append(&vbox);
        hbox.append(&text_view);

        hbox.into()
    }
}
