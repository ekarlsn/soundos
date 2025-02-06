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

#[derive(Clone, Debug)]
pub struct Cursor {
    pub items: Vec<String>,
}

impl Cursor {
    pub fn top(&self) -> &str {
        self.items.first().unwrap()
    }
}
