use std::{
    io::{BufReader, Write},
    rc::Rc,
};

use reqwest::Url;
use rodio::Decoder;

use crate::{
    file_cache,
    menu::{self, Cursor, Menu, MenuAction, MenuOrAction, PressedRightReturn},
    sound::SoundHandle,
};

pub const TOP_NAME: &str = "Pods";

pub struct State {
    pods: Vec<Pod>,
}

struct Pod {
    name: String,
    rss_url: String,
    episodes: Vec<Episode>,
}

struct Episode {
    name: String,
    url: Url,
    file_path: Option<String>,
    current_position: std::time::Duration,
}

pub fn init() -> State {
    State {
        pods: vec![Pod {
            name: "Språket".to_owned(),
            rss_url: "https://api.sr.se/api/rss/pod/itunes/46690".to_owned(),
            episodes: Vec::new(),
        }],
    }
}

pub async fn pressed_right(
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
            "Refresh".to_owned(),
            Menu {
                items: vec![
                    ("Refresh".to_owned(), MenuOrAction::Unknown),
                    ("All".to_owned(), MenuOrAction::Unknown),
                ],
            },
        ),
        ["Refresh"] => {
            sound_handle.say("Refreshing");

            for pod in state.pods.iter_mut() {
                let content = reqwest::get(pod.rss_url.as_str())
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();
                let feed = feed_rs::parser::parse(&content[..]).unwrap();

                pod.episodes.clear();

                for e in feed.entries {
                    let url = e
                        .media
                        .first()
                        .unwrap()
                        .content
                        .first()
                        .unwrap()
                        .url
                        .as_ref()
                        .unwrap()
                        .to_owned();

                    let episode = Episode {
                        name: e.title.unwrap().content,
                        url,
                        file_path: None,
                        current_position: std::time::Duration::from_secs(0),
                    };
                    pod.episodes.push(episode);
                }
            }

            async_std::task::sleep(std::time::Duration::from_secs(5)).await;

            sound_handle.say("Refresh done");

            PressedRightReturn::Nothing
        }
        ["All"] => {
            let pod_names: Vec<String> = state.pods.iter().map(|pod| pod.name.clone()).collect();
            let first = pod_names.first().unwrap().clone();
            let menu_items = pod_names
                .into_iter()
                .map(|name| (name, MenuOrAction::Unknown))
                .collect();
            PressedRightReturn::MenuWithPos(first, Menu { items: menu_items })
        }
        ["All", pod_name] => {
            let pod = state.pods.iter().find(|pod| &pod.name == pod_name).unwrap();
            let episodes = &pod.episodes;
            let Some(first_episode) = episodes.first() else {
                sound_handle.say("No episodes loaded");
                return PressedRightReturn::Nothing;
            };
            let cursor_start = first_episode.name.clone();
            let menu_items = episodes
                .iter()
                .map(|ep| (ep.name.clone(), MenuOrAction::Unknown))
                .collect();
            PressedRightReturn::MenuWithPos(cursor_start, Menu { items: menu_items })
        }
        ["All", _pod_name, _episode_name] => PressedRightReturn::MenuWithPos(
            "Download".to_owned(),
            Menu {
                items: vec![
                    ("Download".to_owned(), MenuOrAction::Unknown),
                    ("Play".to_owned(), MenuOrAction::Unknown),
                    ("Pause".to_owned(), MenuOrAction::Unknown),
                    ("Resume".to_owned(), MenuOrAction::Unknown),
                ],
            },
        ),
        ["All", pod_name, episode_name, "Download"] => {
            let episode = state
                .pods
                .iter_mut()
                .find(|pod| &pod.name == pod_name)
                .unwrap()
                .episodes
                .iter_mut()
                .find(|ep| &ep.name == episode_name)
                .unwrap();
            let url = episode.url.clone();
            sound_handle.say("Downloading");

            let sound_file = reqwest::get(url.clone()).await.unwrap();
            let filename = file_cache::create_file(
                pod_name.to_string(),
                episode_name.to_string(),
                &sound_file.bytes().await.unwrap(),
            );
            episode.file_path = Some(filename);

            sound_handle.say("Download complete");

            PressedRightReturn::Nothing
        }
        ["All", pod_name, episode_name, "Play"] => {
            let Some(file_content) =
                file_cache::read_file(pod_name.to_string(), episode_name.to_string())
            else {
                sound_handle.say("File not found");
                return PressedRightReturn::Nothing;
            };

            sound_handle.say("Playing");
            sound_handle.play_music(file_content);
            PressedRightReturn::Nothing
        }
        ["All", pod_name, episode_name, "Resume"] => {
            sound_handle.resume_music();
            PressedRightReturn::Nothing
        }
        ["All", pod_name, episode_name, "Pause"] => {
            sound_handle.pause_music();
            let episode = state
                .pods
                .iter_mut()
                .find(|pod| &pod.name == pod_name)
                .unwrap()
                .episodes
                .iter_mut()
                .find(|ep| &ep.name == episode_name)
                .unwrap();
            episode.current_position = sound_handle.music_sink.get_pos();
            PressedRightReturn::Nothing
        }
        unhandled => panic!("Unhandled cursor state: {unhandled:?}"),
    }
}
