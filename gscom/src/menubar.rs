use gio::{Menu, MenuItem};

pub fn create_menubar() -> Menu {
    let file_menu = {
        let about_menu_item = MenuItem::new(Some("About"), Some("app.about"));
        let quit_menu_item = MenuItem::new(Some("Quit"), Some("app.quit"));

        let file_menu = Menu::new();
        file_menu.append_item(&about_menu_item);
        file_menu.append_item(&quit_menu_item);
        file_menu
    };

    let menubar = Menu::new();
    menubar.append_submenu(Some("File"), &file_menu);
    menubar
}