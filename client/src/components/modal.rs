use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons};
use serde::{Serialize, Deserialize};
use tp_models::podcast::Episode;

use crate::PLAYER_STATUS;

#[derive(Serialize, Deserialize)]
pub struct PlayBoxInfo {
    pub playlist: Option<Vec<Episode>>,
    pub current: usize,
    pub display: bool,
    pub pause: bool,
}

pub fn PlayBox(cx: Scope) -> Element {

    let current_content: &UseState<usize> = use_state(&cx, || usize::MAX);
    let status = use_atom_ref(&cx, PLAYER_STATUS);
    let status_content = status.read().current;

    let playlist = if status.read().playlist.is_none() {
        vec![]
    } else {
        status.read().playlist.clone().unwrap()
    };

    // use this check to reload the play box source
    if &status_content != current_content.get() {
        current_content.set(status_content);
        return cx.render(rsx! {
            div {}
        });
    }

    if playlist.get(status.read().current).is_none() {
        return cx.render(rsx! {
            div {
                class: "fixed bottom-12 left-2 rounded-full w-10 h-10 
                bg-white dark:bg-gray-900 hover:bg-black dark:hover:bg-white",
                button {
                    class: "justify-center w-full h-full text-black dark:text-white hover:text-white dark:hover:text-black",
                    Icon {
                        class: "h-full w-full",
                        icon: fa_solid_icons::FaCirclePlay,
                    }
                }
            }
        });
    }

    let info = playlist.get(status.read().current).unwrap();

    let player_hidden = if !status.read().display {
        "hidden"
    } else { "hidden sm:block" };
    let icon_hidden = if status.read().display {
        "hidden"
    } else { "hidden sm:block" };

    // let eval_script = use_eval::<&str>(&cx);

    let full_title = if info.title.is_empty() {
        &info.title_original
    } else {
        &info.title
    };
    let simple_title = if full_title.len() > 32 {
        format!("{} ...", &full_title[0..32])
    } else {
        full_title.to_string()
    };

    cx.render(rsx! {
        div {
            class: "{icon_hidden} fixed bottom-12 left-2 rounded-full w-10 h-10 
            bg-white dark:bg-gray-900 hover:bg-black dark:hover:bg-white",
            button {
                class: "justify-center w-full h-full text-black dark:text-white hover:text-white dark:hover:text-black",
                onclick: move |_| {
                    status.write().display = true;
                },
                Icon {
                    class: "w-full h-full",
                    icon: fa_solid_icons::FaCirclePlay,
                }
            }
        }
        div {
            class: "{player_hidden} fixed bottom-8 left-0 w-1/3 h-20 rounded
            bg-white dark:bg-gray-900 shadow-2xl
            px-2 py-2 z-40
            ",
            div {
                class: "flex h-full gap-2",
                div {
                    class: "flex-initial w-16",
                    img {
                        class: "h-full rounded",
                        src: "{info.thumbnail}",
                    }
                }
                div {
                    class: "grow relative",
                    div {
                        span {
                            class: "text-black dark:text-white",
                            title: "{info.title}",
                            "{simple_title}"
                        }
                        span {
                            class: "absolute right-0",
                            div {
                                class: "flex items-center justify-center",
                                div {
                                    class: "inline-flex",
                                    role: "group",
                                    button {
                                        class: "rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type: "button",
                                        Icon {
                                            icon: fa_solid_icons::FaStar,
                                        }   
                                    }
                                    button {
                                        class: "rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type: "button",
                                        onclick: move |_| {
                                            status.write().display = false;
                                        },
                                        Icon {
                                            icon: fa_solid_icons::FaCircleMinus,
                                        }   
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute bottom-0 w-full",
                        // input {
                        //     class: "w-full h-1",
                        //     r#type: "range",
                        //     min: "0",
                        //     max: "60",
                        //     step: "1",
                        //     value: "0"
                        // }
                        audio {
                            id: "podcast-player",
                            class: "w-full h-8",
                            controls: "controls",
                            "controlsList": "nodownload",
                            autoplay: "true",
                            "oncontextmenu": "return false",
                            onpause: |e| {
                                log::info!("{:?}", e);
                            },
                            source {
                                id: "audio-source",
                                src: "{info.audio}",
                                "type": "audio/mp3"
                            }
                        }
                        script { src: "/script/audio.js" }
                    }
                }
            }
        }
    })
}