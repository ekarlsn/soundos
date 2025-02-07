#[derive(Clone, Debug)]
pub enum MenuOrAction {
    Menu(Menu),
    Action(MenuAction),
    Unknown,
}

#[derive(Clone, Debug)]
pub enum MenuAction {
    Play,
    Pause,
    None,
    PodAction(PodAction),
}

#[derive(Clone, Debug)]
pub enum PodAction {
    Play,
    Pause,
}

#[derive(Clone, Debug)]
pub struct Menu {
    pub items: Vec<(String, MenuOrAction)>,
}

impl Menu {
    pub fn attach_menu_at(&mut self, cursor: &Cursor, menu_to_insert: Menu) {
        let mut submenu: Option<&mut Menu> = Some(self);
        for selected in cursor.items.iter().take(cursor.items.len() - 1) {
            submenu = submenu
                .unwrap()
                .items
                .iter_mut()
                .find(|(item_name, _)| item_name == selected)
                .map(|(_, menu)| match menu {
                    MenuOrAction::Menu(m) => m,
                    _ => panic!("The cursor pointed to a thing that's not a menu! {selected}"),
                })
        }
        let insert_place_name = cursor.last();
        let insert_tuple = submenu
            .unwrap()
            .items
            .iter_mut()
            .find(|(item_name, _)| item_name == insert_place_name)
            .unwrap();

        insert_tuple.1 = MenuOrAction::Menu(menu_to_insert);
    }
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub items: Vec<String>,
}

impl Cursor {
    pub fn top(&self) -> &str {
        self.items.first().unwrap()
    }

    pub fn last(&self) -> &str {
        self.items.last().unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum PressedRightReturn {
    MenuWithPos(String, Menu),
    Say(String),
    Nothing,
}
