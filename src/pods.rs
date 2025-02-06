use crate::menu::{self, Cursor, Menu, MenuAction, MenuOrAction};

pub const TOP_NAME: &str = "Pods";

pub struct State {
    pub id: u32,
}

pub fn init() -> (String, State) {
    ("Pods".to_owned(), State { id: 5 })
}

// Returs the root level name its menu
pub fn get_pods_menu(state: &State) -> (Vec<String>, String, MenuOrAction) {
    let id = state.id;
    (
        vec!["Pods".to_owned(), "Software Unscripted".to_owned()],
        "Pods".to_string(),
        MenuOrAction::Menu(Menu {
            items: vec![
                (
                    "Software Unscripted".to_string(),
                    MenuOrAction::Menu(Menu {
                        items: vec![
                            (
                                "Play".to_string(),
                                MenuOrAction::Action(MenuAction::PodAction(menu::PodAction::Pause)),
                            ),
                            (
                                "Pause".to_string(),
                                MenuOrAction::Action(MenuAction::PodAction(menu::PodAction::Pause)),
                            ),
                            (
                                format!("Count {id}"),
                                MenuOrAction::Action(MenuAction::None),
                            ),
                        ],
                    }),
                ),
                (
                    "Testing in production".to_string(),
                    MenuOrAction::Menu(Menu {
                        items: vec![
                            ("Play".to_string(), MenuOrAction::Action(MenuAction::Play)),
                            ("Pause".to_string(), MenuOrAction::Action(MenuAction::Pause)),
                        ],
                    }),
                ),
            ],
        }),
    )
}

pub fn pressed_right(cursor: &Cursor, state: &mut State) {
    let cursor: &Vec<&str> = &cursor.items.iter().map(|s| s.as_str()).collect();
    match cursor.as_slice() {
        ["Pods"] => {}
        ["Pods", pod_name] => {}
        _ => {}
    }
}
