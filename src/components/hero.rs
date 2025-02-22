use crate::menu::{Cursor, Menu, MenuAction, MenuOrAction, PressedRightReturn};
use crate::pods;
use crate::sound::SoundHandle;
use dioxus::prelude::*;
use std::rc::Rc;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
#[allow(non_snake_case)]
pub fn Hero() -> Element {
    let mut sound_handle = use_signal(SoundHandle::new);
    let mut pod_state = use_signal(pods::init);

    // The menu navigation
    let mut menu = use_signal(|| Menu {
        items: vec![
            (pods::TOP_NAME.to_owned(), MenuOrAction::Unknown),
            (
                "Settings".to_owned(),
                MenuOrAction::Menu(Menu {
                    items: vec![("Dummy".to_owned(), MenuOrAction::Action(MenuAction::None))],
                }),
            ),
        ],
    });

    // Cursor
    let cursor = use_signal(|| Cursor {
        items: vec!["Pods".to_owned()],
    });

    // Derived from menu + cursor
    let (active_menu, active_index) = {
        let cursor = cursor;
        let menu = menu;
        let active_menu = get_items_deep(&cursor.read().items, &menu.read()).unwrap();
        let active_index = {
            let last_cursor = cursor.read().items.last().unwrap().clone();
            active_menu
                .iter()
                .position(|item| item == last_cursor.as_str())
                .unwrap()
        };
        (active_menu, active_index)
    };

    // Lock for all interactions
    let mut mutex = use_signal(|| Rc::new(async_mutex::Mutex::new(())));

    let mut breed = use_signal(|| "https://images.dog.ceo/breeds/shiba/shiba-1.jpg".to_owned());

    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            ul {
                li {
                    BreedPic { breed }
                }
                li {
                    button {
                        id: "Up",
                        onclick: move |_| {
                            let mut cursor = cursor;
                            let mutex = mutex.write().clone();
                            async move {
                                if let Some(_lock) = mutex.try_lock() {
                                    move_menu_position(&mut cursor.write(), &mut menu, Dir::Up, &mut sound_handle, &mut pod_state).await;
                                }
                            }
                        },
                        "Up"
                    }
                }
                li {
                    button {
                        id: "Down",
                        onclick: move |_| {
                            let mut cursor = cursor;
                            let mutex = mutex.write().clone();
                            async move {
                                if let Some(_lock) = mutex.try_lock() {
                                    move_menu_position(&mut cursor.write(), &mut menu, Dir::Down,  &mut sound_handle, &mut pod_state).await;
                                }
                            }
                        },
                        "Down"
                    }
                }
                li {
                    button {
                        id: "Left",
                        onclick: move |_| {
                            let mut cursor = cursor;
                            let mutex = mutex.write().clone();
                            async move {
                                if let Some(_lock) = mutex.try_lock() {
                                    move_menu_position(&mut cursor.write(), &mut menu, Dir::Left,  &mut sound_handle, &mut pod_state).await;
                                }
                            }
                        },
                        "Left"
                    }
                    button {
                        id: "Right",
                        onclick: move |_| {
                            let mut cursor = cursor;
                            let mutex = mutex.write().clone();
                            async move {
                                if let Some(_lock) = mutex.try_lock() {
                                    println!("Lock aquired");
                                    move_menu_position(&mut cursor.write(), &mut menu, Dir::Right,  &mut sound_handle, &mut pod_state).await;
                                    println!("Releasing lock");
                                } else {
                                    println!("Lock already aquired");
                                }
                            }
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

async fn move_menu_position(
    cursor: &mut Cursor,
    menu: &mut Signal<Menu>,
    direction: Dir,
    sound_handle: &mut Signal<SoundHandle>,
    pod_state: &mut Signal<pods::State>,
) {
    if direction == Dir::Right {
        let action = match cursor.top() {
            pods::TOP_NAME => {
                pods::pressed_right(cursor, &mut pod_state.write(), &mut sound_handle.write()).await
            }
            "Settings" => PressedRightReturn::Say("No settings".to_owned()),
            _ => panic!("Pressed something unknown"),
        };

        match action {
            PressedRightReturn::Nothing => {}
            PressedRightReturn::MenuWithPos(pos, submenu) => {
                sound_handle.write().say(&pos);
                menu.write().attach_menu_at(cursor, submenu);
                cursor.items.push(pos);
            }
            PressedRightReturn::Say(say) => sound_handle.write().say(&say),
        }
    } else {
        sound_handle.write().pause_music();
        let say = update_cursor_uld(cursor, &menu.read(), direction);
        if let Some(say) = say {
            sound_handle.write().say(&say);
        }
    }
}

// Return what to say
fn update_cursor_uld(cursor: &mut Cursor, menu: &Menu, direction: Dir) -> Option<String> {
    let cursor: &mut Vec<String> = cursor.items.as_mut();
    println!("Moving cursor {:?}", direction);
    if let Dir::Left = direction {
        println!("Got it writable");
        if cursor.len() > 1 {
            cursor.pop();
            println!("popped it!");
            let new_menu_selection = cursor.last().unwrap();
            return Some(new_menu_selection.to_owned());
        } else {
            println!("Can't pop it!");
            return None;
        }
    };

    // if let Dir::Right = direction {
    // let mut submenu = Some(menu);
    // {
    // let cursor_read = cursor.read();
    // for (i, selected) in cursor_read.iter().enumerate() {
    // submenu = match get_menu_selection(selected, submenu.unwrap()) {
    // MenuOrAction::Menu(submenu) => Some(submenu),
    // MenuOrAction::Action(action) => {
    // if i == cursor.len() - 1 {
    // Trying to go right on an action
    // return (None, Some(action.clone()));
    // } else {
    // panic!(
    // "Cursor is pointing at a menu item that does not exist: {selected}"
    // );
    // }
    // }
    // };
    // }
    // }
    // let mut cursor_write = cursor.write();
    // let new_menu_item = submenu.unwrap().items.first().unwrap().0.clone();
    // cursor_write.push(new_menu_item.clone());
    // return (Some(new_menu_item), None);
    // }

    let active_list = get_items_deep(cursor.as_slice(), menu).unwrap();
    let last_cursor_index = cursor.len() - 1;
    for (i, item) in active_list.iter().enumerate() {
        if item == cursor.last().unwrap() {
            match direction {
                Dir::Up => {
                    if i == 0 {
                        // Trying to go up, but already at top
                        return None;
                    } else {
                        let new_menu_item = active_list[i - 1].clone();
                        cursor[last_cursor_index] = new_menu_item.clone();
                        return Some(new_menu_item);
                    }
                }
                Dir::Down => {
                    if i == active_list.len() - 1 {
                        // Trying to go down, but already at bottom
                        return None;
                    } else {
                        let new_menu_item = active_list[i + 1].clone();
                        cursor[last_cursor_index] = new_menu_item.clone();
                        return Some(new_menu_item);
                    }
                }
                _ => {}
            }
            return None;
        }
    }
    None
}

fn get_items_deep(cursor: &[String], menu: &Menu) -> Option<Vec<String>> {
    let mut submenu = Some(menu);
    for selected in cursor.iter().take(cursor.len() - 1) {
        submenu = match get_menu_selection(selected, submenu.unwrap()) {
            MenuOrAction::Menu(submenu) => Some(submenu),
            MenuOrAction::Action(_) => return None,
            MenuOrAction::Unknown => return None,
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

fn get_menu_selection<'a>(menu_item: &str, menu: &'a Menu) -> &'a MenuOrAction {
    menu.items
        .iter()
        .find(|(item, _)| item == &menu_item)
        .map(|(_, menu_or_action)| menu_or_action)
        .unwrap()
}

#[component]
fn BreedPic(breed: Signal<String>) -> Element {
    // This resource will restart whenever the breed changes
    let mut fut = use_resource(move || async move {
        #[derive(serde::Deserialize, Debug)]
        struct DogApi {
            message: String,
        }

        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await
            .unwrap()
            .json::<DogApi>()
            .await
    });

    match fut.read_unchecked().as_ref() {
        Some(Ok(resp)) => rsx! {
            div {
                button { onclick: move |_| fut.restart(), padding: "5px", background_color: "gray", color: "white", border_radius: "5px", "Click to fetch another doggo" }
                img { max_width: "500px", max_height: "500px", src: "{resp.message}" }
            }
        },
        Some(Err(_)) => rsx! { "loading image failed" },
        None => rsx! { "loading image 2..." },
    }
}
