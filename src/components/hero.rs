use crate::menu::{Cursor, Menu, MenuAction, MenuOrAction};
use crate::pods;
use dioxus::prelude::*;
use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;
use std::{path::Path, rc::Rc};

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    // Setup TTS engine
    let model = use_hook(|| {
        let config_path = "en_US-libritts_r-medium.onnx.json";
        piper_rs::from_config_path(Path::new(config_path)).unwrap()
    });
    let synth = Rc::new(PiperSpeechSynthesizer::new(model).unwrap());
    let synth2 = synth.clone();
    let synth3 = synth.clone();
    let synth4 = synth.clone();
    let synth5 = synth.clone();

    // Sound sink
    let mut sound_sink = use_signal(|| {
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        Rc::new(SoundSink {
            stream,
            handle,
            sink,
        })
    });

    let mut pod_state = use_signal(|| pods::State { id: 5 });

    // The menu navigation
    let menu = use_signal(|| Menu {
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

    let mut cursor = use_signal(|| Cursor {
        items: vec!["Pods".to_owned()],
    });

    let active_menu = get_items_deep(&cursor.read().items, &menu.read()).unwrap();
    let last_cursor = cursor.read().items.last().unwrap().clone();
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
                        onclick: move |_| {
                            move_menu_position(&mut cursor.write().items, &menu.read(), Dir::Up, &synth, &mut sound_sink, &mut pod_state);
                        },
                        "Up"
                    }
                }
                li {
                    button {
                        id: "Down",
                        onclick: move |_| {
                            move_menu_position(&mut cursor.write().items, &menu.read(), Dir::Down, &synth2, &mut sound_sink, &mut pod_state);
                        },
                        "Down"
                    }
                }
                li {
                    button {
                        id: "Left",
                        onclick: move |_| {
                            move_menu_position(&mut cursor.write().items, &menu.read(), Dir::Left, &synth3, &mut sound_sink, &mut pod_state);
                        },
                        "Left"
                    }
                    button {
                        id: "Right",
                        onclick: move |_| {
                            move_menu_position(&mut cursor.write().items, &menu.read(), Dir::Right, &synth4, &mut sound_sink, &mut pod_state);
                        },
                        "Right"
                    }
                }
                button {
                    id: "Speak",
                    onclick: move |_| {
                        let current_text = cursor.read().items.last().map_or("Failed to find text to read", |v| v).to_owned();
                        speak(&synth5, &mut sound_sink, &current_text );
                    },
                    "Speak"
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

fn move_menu_position(
    cursor: &mut Cursor,
    menu: &Menu,
    direction: Dir,
    synth: &PiperSpeechSynthesizer,
    sound_sink: &mut Signal<Rc<SoundSink>>,
    pod_state: Signal<pods::State>,
) {
    if direction == Dir::Right {
        match cursor.top() {
            pods::TOP_NAME => {
                pods::pressed_right(cursor, &mut pod_state.write());
            }
            _ => panic!("Pressed something unknown"),
        }
    } else {
        let say = update_cursor_uld(cursor, menu, direction);
        if let Some(say) = say {
            speak(synth, sound_sink, &say);
        }
    }
}

// Return what to say, and what to do
fn update_cursor_uld(
    cursor: &mut Cursor,
    menu: &Menu,
    direction: Dir,
) -> (Option<String>, Option<MenuAction>) {
    let cursor: &mut Vec<String> = cursor.items.as_mut();
    println!("Moving cursor {:?}", direction);
    if let Dir::Left = direction {
        println!("Got it writable");
        if cursor.len() > 1 {
            cursor.pop();
            println!("popped it!");
            let new_menu_selection = cursor.last().unwrap();
            return (Some(new_menu_selection.to_owned()), None);
        } else {
            println!("Can't pop it!");
            return (None, None);
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
                            Trying to go right on an action
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

    let active_list = get_items_deep(&cursor.read(), menu).unwrap();
    let last_cursor_index = cursor.read().len() - 1;
    let mut cursor = cursor.write();
    for (i, item) in active_list.iter().enumerate() {
        if item == cursor.last().unwrap() {
            match direction {
                Dir::Up => {
                    if i == 0 {
                        // Trying to go up, but already at top
                        return (None, None);
                    } else {
                        let new_menu_item = active_list[i - 1].clone();
                        cursor[last_cursor_index] = new_menu_item.clone();
                        return (Some(new_menu_item), None);
                    }
                }
                Dir::Down => {
                    if i == active_list.len() - 1 {
                        // Trying to go down, but already at bottom
                        return (None, None);
                    } else {
                        let new_menu_item = active_list[i + 1].clone();
                        cursor[last_cursor_index] = new_menu_item.clone();
                        return (Some(new_menu_item), None);
                    }
                }
                _ => {}
            }
            return (None, None);
        }
    }
    return (None, None);
}

fn get_items_deep(cursor: &[String], menu: &Menu) -> Option<Vec<String>> {
    let mut submenu = Some(menu);
    for selected in cursor.iter().take(cursor.len() - 1) {
        submenu = match get_menu_selection(selected, submenu.unwrap()) {
            MenuOrAction::Menu(submenu) => Some(submenu),
            MenuOrAction::Action(_) => return None,
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

struct SoundSink {
    stream: rodio::OutputStream,
    handle: rodio::OutputStreamHandle,
    sink: rodio::Sink,
}

fn speak(synth: &PiperSpeechSynthesizer, sound_sink: &mut Signal<Rc<SoundSink>>, text: &str) {
    println!("Trying to speak");

    let mut samples: Vec<f32> = Vec::new();
    let audio = synth.synthesize_parallel(text.to_string(), None).unwrap();
    for result in audio {
        samples.append(&mut result.unwrap().into_vec());
    }

    let buf = SamplesBuffer::new(1, 22050, samples);
    let sound_sink = sound_sink.write();
    sound_sink.sink.append(buf);

    println!("Done speaking");
}
