use gtk::{prelude::*, *};
use crate::panel::PanelTrait;

pub struct StatusPanel;

impl PanelTrait for StatusPanel {
    fn build_panel() -> Widget {
        let image = Image::from_file("assets/unplug.svg");
        let sent_label = Label::new(Some("Send"));
        let sent_text = Text::builder().build();
        sent_text.set_text("0");
        let recv_label = Label::new(Some("Recv"));
        let recv_text = Text::builder().build();
        recv_text.set_text("0");

        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.append(&image);
        hbox.append(&sent_label);
        hbox.append(&sent_text);
        hbox.append(&recv_label);
        hbox.append(&recv_text);

        hbox.into()
    }
}
