use crate::{
    menu::{self, Cursor, Menu, MenuAction, MenuOrAction, PressedRightReturn},
    sound::SoundHandle,
};

pub const TOP_NAME: &str = "Pods";

pub struct State {
    pub id: u32,
}

pub fn init() -> (String, State) {
    ("Pods".to_owned(), State { id: 5 })
}

pub fn pressed_right(
    cursor: &Cursor,
    state: &mut State,
    sound_handle: &mut SoundHandle,
) -> PressedRightReturn {
    if cursor.top() != TOP_NAME {
        panic!("Why did you call me with something unrelated?");
    }
    let cursor: &Vec<&str> = &cursor.items.iter().skip(1).map(|s| s.as_str()).collect();
    match cursor.as_slice() {
        [] => PressedRightReturn::MenuWithPos(
            "Lang".to_owned(),
            Menu {
                items: vec![
                    ("Lang".to_owned(), MenuOrAction::Unknown),
                    ("Run".to_owned(), MenuOrAction::Unknown),
                    ("Climb".to_owned(), MenuOrAction::Unknown),
                ],
            },
        ),
        [_] => PressedRightReturn::MenuWithPos(
            "Play".to_owned(),
            Menu {
                items: vec![
                    ("Play".to_owned(), MenuOrAction::Unknown),
                    ("Pause".to_owned(), MenuOrAction::Unknown),
                ],
            },
        ),
        [pod_name, "Play"] => {
            // Actually start playing, and save the state, maybe like a music sink, into the state
            PressedRightReturn::Say(format!("Ok, I'll start playing {pod_name}"))
        }
        _ => PressedRightReturn::Nothing,
    }
}
