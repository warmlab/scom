pub trait PanelTrait {
    fn new() -> Self;
    fn build_panel(&self) -> gtk::Widget;
}