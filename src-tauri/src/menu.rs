use tauri::{CustomMenuItem, Menu, Submenu};

pub struct MenuBar {
    menu: Menu,
}

impl MenuBar {
    pub fn as_menu(self) -> Menu {
        self.menu
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        let database_submenu = Submenu::new(
            "Database",
            Menu::new().add_item(CustomMenuItem::new("register_database", "Create")),
        );

        MenuBar {
            menu: Menu::new().add_submenu(database_submenu),
        }
    }
}
