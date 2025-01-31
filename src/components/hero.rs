use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    let menu = use_signal(|| Menu {
        items: vec![
            (
                "Pods".to_owned(),
                MenuOrAction::Menu(Menu {
                    items: vec![
                        ("Play".to_string(), MenuOrAction::Action(MenuAction::Play)),
                        ("Pause".to_string(), MenuOrAction::Action(MenuAction::Pause)),
                    ],
                }),
            ),
            (
                "Settings".to_owned(),
                MenuOrAction::Menu(Menu {
                    items: vec![("Dummy".to_owned(), MenuOrAction::Action(MenuAction::None))],
                }),
            ),
        ],
    });

    let mut cursor = use_signal(|| vec!["Pods".to_owned(), "Play".to_owned()]);

    let active_menu = get_items_deep(&cursor.read(), &menu.read()).unwrap();
    let last_cursor = cursor.last().unwrap();
    let active_index = active_menu
        .iter()
        .position(|item| item == last_cursor.as_str())
        .unwrap();

    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            ul {
                li {
                    button {
                        id: "Up",
                        onclick: move |event| {
                            move_cursor(&mut cursor, &menu.read(), Dir::Up);
                        },
                        "Up"
                    }
                }
                li {
                    button {
                        id: "Down",
                        onclick: move |event| {
                            move_cursor(&mut cursor, &menu.read(), Dir::Down);
                        },
                        "Down"
                    }
                }
                li {
                    button {
                        id: "Left",
                        onclick: move |event| {
                            move_cursor(&mut cursor, &menu.read(), Dir::Left);
                        },
                        "Left"
                    }
                    button {
                        id: "Right",
                        onclick: move |event| {
                            move_cursor(&mut cursor, &menu.read(), Dir::Right);
                        },
                        "Right"
                    }
                }
            }
            div { id: "menu",
                ul {
                    for (i, menu_item) in active_menu.iter().enumerate() {
                        if i == active_index {
                            li {
                                key: menu_item,
                                class: "active",
                                "* {menu_item}"
                            }
                        } else {
                            li {
                                key: menu_item,
                                "{menu_item}"
                            }
                        }
                    }
                }
            }
        }
    }
}

enum MenuOrAction {
    Menu(Menu),
    Action(MenuAction),
}

enum MenuAction {
    Play,
    Pause,
    None,
}

struct Menu {
    items: Vec<(String, MenuOrAction)>,
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn move_cursor(cursor: &mut Signal<Vec<String>>, menu: &Menu, direction: Dir) {
    println!("Moving cursor {:?}", direction);
    if let Dir::Left = direction {
        let mut cursor = cursor.write();
        println!("Got it writable");
        if cursor.len() > 1 {
            cursor.pop();
            println!("popped it!");
        } else {
            println!("Can't pop it!");
        }
        return;
    };

    if let Dir::Right = direction {
        let mut submenu = Some(menu);
        for selected in cursor.read().iter() {
            submenu = match get_submenu(selected, submenu.unwrap()) {
                Some(submenu) => Some(submenu),
                None => return,
            };
        }
        let mut cursor = cursor.write();
        cursor.push(submenu.unwrap().items.first().unwrap().0.clone());
        return;
    }

    let active_list = get_items_deep(&cursor.read(), menu).unwrap();
    let last_cursor_index = cursor.read().len() - 1;
    let mut cursor = cursor.write();
    for (i, item) in active_list.iter().enumerate() {
        if item == cursor.last().unwrap() {
            match direction {
                Dir::Up => {
                    if i == 0 {
                        return;
                    } else {
                        cursor[last_cursor_index] = active_list[i - 1].clone();
                    }
                }
                Dir::Down => {
                    if i == active_list.len() - 1 {
                        return;
                    } else {
                        cursor[last_cursor_index] = active_list[i + 1].clone();
                    }
                }
                _ => {}
            }
            return;
        }
    }
}

fn get_items_deep(cursor: &[String], menu: &Menu) -> Option<Vec<String>> {
    let mut submenu = Some(menu);
    for selected in cursor.iter().take(cursor.len() - 1) {
        submenu = match get_submenu(selected, submenu.unwrap()) {
            Some(submenu) => Some(submenu),
            None => return None,
        };
    }
    Some(
        submenu
            .unwrap()
            .items
            .iter()
            .map(|(item, _)| item.clone())
            .collect(),
    )
}

fn get_submenu<'a>(menu_item: &str, menu: &'a Menu) -> Option<&'a Menu> {
    menu.items
        .iter()
        .find(|(item, _)| item == &menu_item)
        .and_then(|(_, menu_or_action)| match menu_or_action {
            MenuOrAction::Menu(menu) => Some(menu),
            _ => None,
        })
}
