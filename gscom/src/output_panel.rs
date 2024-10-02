use gtk::{prelude::*, *};

use crate::panel::PanelTrait;
use crate::port_panel::{self, PortPanel};

pub struct OutputPanel {
    //text_buffer: TextBuffer,
}

impl OutputPanel {
    fn build_control(&self) -> Widget {
        let vbox = Box::new(Orientation::Vertical, 10);
        let button = Button::builder()
            .label("Clear")
            .build();

        let hbox = Box::new(Orientation::Horizontal, 10);
        let switch = Switch::new();
        switch.set_active(false);
        let label = Label::new(Some("Show as HEX code"));
        hbox.append(&switch);
        hbox.append(&label);

        vbox.append(&hbox);
        vbox.append(&button);

        vbox.into()
    }
}

impl PanelTrait for OutputPanel {
    fn new() -> Self {
        OutputPanel {}
    }

    fn build_panel(&self) -> Widget {
        // let input = Entry::builder().placeholder_text("I will contain ones...").build();
        let text_view = TextView::builder().editable(false).cursor_visible(false).overwrite(true).hexpand(true).vexpand(true).build();
        let buffer = text_view.buffer(); //.expect("Failed to get buffer");
        let mut iter = buffer.iter_at_line(0).unwrap();
        buffer.insert(&mut iter, "text");
        let (start, end) = buffer.bounds();
        println!("text view iter=[{:?}], start=[{:?}], end=[{:?}]", iter, start, end);

        let vbox = Box::new(Orientation::Vertical, 10);
        let port_panel = PortPanel::new();
        let port_box = port_panel.build_panel();
        let control_box = OutputPanel::build_control(self);
        vbox.append(&port_box);
        vbox.append(&control_box);

        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.append(&vbox);
        hbox.append(&text_view);

        hbox.into()
    }
}
