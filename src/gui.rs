use crate::{
    GZ_VERSION, do_extract_ui, do_repack, is_ready_to_patch, iso_tools::GameVersion, patcher,
};
use dioxus::prelude::*;
use std::sync::mpsc;

const FAVICON: Asset = asset!("/icons/icon.ico");
const CSS: &str = include_str!("../assets/gz.css");

const SUPPORTED_VERSIONS: [GameVersion; 2] = [GameVersion::NTSC1_0, GameVersion::JP];

pub fn do_gui() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "favicon", href: FAVICON }
        
        style { "{CSS}" }
        GZ {}
    }
}

fn do_patch<T: FnMut(u8)>(version: GameVersion, cb: &mut T) -> anyhow::Result<()> {
    if !is_ready_to_patch(version) {
        return Err(anyhow::format_err!(
            "The extract for the {version} version isn't ready for patching. Please restart the program to redo extraction."
        ));
    }
    patcher::do_gz_patches(version)?;

    let repack_iso = true;

    if repack_iso {
        do_repack(version, cb)?;
    }
    Ok(())
}

#[component]
pub fn GZ() -> Element {
    rsx! {
        div {
            class: "container",
            Title {}
            div {
                class: "columns",
                for ver in SUPPORTED_VERSIONS {
                    VersionCol { version: ver }
                }
            }
        }
    }
}

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            class: "header",
            h1 {
                class: "main-title",
                "SSGZ Version {GZ_VERSION}"
            }
            h2 {
                class: "sub-title",
                "A Practice ROM Hack for Skyward Sword"
            }
        }
    }
}

#[component]
pub fn VersionCol(version: GameVersion) -> Element {
    let mut progress_percentage = use_signal(|| 0u8);
    let mut is_busy = use_signal(|| false);
    let mut showing_info = use_signal(|| false);
    let mut info = use_signal(|| "".to_owned());
    let mut can_patch = use_signal(|| is_ready_to_patch(version));

    let ext_status = if *can_patch.read() {
        "Extract is ready for patching"
    } else {
        "No extract found"
    };

    // Channel for receiving updates from the background thread
    let mut update_receiver = use_signal(|| None::<mpsc::Receiver<FileIOStatus>>);

    use_effect(move || {
        if *is_busy.read() {
            spawn(async move {
                while *is_busy.read() {
                    if let Some(receiver) = update_receiver.read().as_ref() {
                        while let Ok(update) = receiver.try_recv() {
                            match update {
                                FileIOStatus::Progress(prog) => {
                                    progress_percentage.set(prog);
                                }
                                FileIOStatus::Completed => {
                                    if *can_patch.read() {
                                        info.set(
                                            "Patching done, happy speedrunning! Press Z and C simultaneously to access practice menus!".to_string()
                                        );
                                    } else {
                                        info.set(
                                            "Extraction complete. Click `Write Patched ISO` to create your practice ROM.".to_string()
                                        );
                                    }
                                    is_busy.set(false);
                                    showing_info.set(true);
                                    can_patch.set(is_ready_to_patch(version));
                                    return;
                                }
                                FileIOStatus::Failed(err) => {
                                    if *can_patch.read() {
                                        info.set(format!("Patching failed: {}", err));
                                    } else {
                                        info.set(format!("Extraction failed: {}", err));
                                    }
                                    is_busy.set(false);
                                    showing_info.set(true);
                                    return;
                                }
                            }
                        }
                    }

                    // Ensure the application doesn't freeze due to "not responding"
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            });
        }
    });

    let handle_click = move |_evt| {
        if *is_busy.read() {
            return;
        }

        is_busy.set(true);
        progress_percentage.set(0);
        let patch = *can_patch.read();

        let (sender, receiver) = mpsc::channel();
        update_receiver.set(Some(receiver));

        // Need to use a background thread so the UI remains active
        std::thread::spawn(move || {
            let sender_clone = sender.clone();
            let set_progress = move |prog| {
                let _ = sender_clone.send(FileIOStatus::Progress(prog));
            };

            let result = if patch {
                do_patch(version, &mut |prog| {
                    set_progress(prog);
                })
            } else {
                do_extract_ui(version, &mut |prog| {
                    set_progress(prog);
                })
            };

            match result {
                Ok(_) => {
                    let _ = sender.send(FileIOStatus::Completed);
                }
                Err(e) => {
                    let _ = sender.send(FileIOStatus::Failed(e.to_string()));
                }
            }
        });
    };

    rsx! {
        div {
            class: "column",
            div {
                class: "column-text",
                "{version.to_string()} Version"
            }
            div {
                class: "column-text",
                "{ext_status}"
            }
            button {
                class: if *can_patch.read() {
                    "btn patch-btn"
                } else {
                    "btn extract-btn"
                },
                onclick: handle_click,
                disabled: *is_busy.read(),
                match (*can_patch.read(), *is_busy.read()) {
                    (true, true) => "Patching...",
                    (true, false) => "Write Patched ISO",
                    (false, true) => "Extracting...",
                    (false, false) => "Extract ISO",
                }
            }
        }

        // Progress bar popup
        if *is_busy.read() {
            div {
                class: "popup-overlay",
                div {
                    class: "popup-content",
                    h3 { if *can_patch.read() {
                            "Patching {version.to_string()}..."
                        } else {
                            "Extracting {version.to_string()}..."
                        }
                    }
                    div {
                        class: "progress-container",
                        div {
                            class: "progress-bar",
                            style: "width: {progress_percentage}%"
                        }
                    }
                    div {
                        class: "progress-text",
                        "{progress_percentage}%"
                    }
                }
            }
        }

        // Info popup
        if *showing_info.read() {
            div {
                class: "popup-overlay",
                div {
                    class: "popup-content",
                    h3 { "{*info.read()}" }
                    button {
                        class: "btn extract-btn",
                        onclick: move |_evt| {
                            showing_info.set(false);
                        },
                        "OK"
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum FileIOStatus {
    Progress(u8),
    Completed,
    Failed(String),
}
