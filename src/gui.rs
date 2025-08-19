use crate::{ 
    do_extract_ui, do_repack, is_ready_to_patch, iso_tools::GameVersion, patcher, updater::*,
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
        UpdateChecker {}
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
fn UpdateChecker() -> Element {
    let mut is_checking = use_signal(|| false);
    let mut pending_update = use_signal(|| None::<String>);
    let mut show_confirm_popup = use_signal(|| false);
    let mut is_downloading = use_signal(|| false);
    let mut update_complete = use_signal(|| false);
    let mut showing_info = use_signal(|| false);
    let mut info = use_signal(|| "".to_owned());
    let mut update_receiver = use_signal(|| None::<mpsc::Receiver<UpdateStatus>>);

    use_effect(move || {
        is_checking.set(true);
        
        let (sender, receiver) = mpsc::channel();
        update_receiver.set(Some(receiver));

        // Spawn the update check thread
        std::thread::spawn(move || {
            match check_for_update() {
                Ok(Some(asset_name)) => {
                    let _ = sender.send(UpdateStatus::UpdateAvailable(asset_name));
                }
                Ok(None) => {
                    let _ = sender.send(UpdateStatus::NoUpdate);
                }
                Err(e) => {
                    let _ = sender.send(UpdateStatus::Failed(format!("Update check failed: {}", e)));
                }
            }
        });
    });

    // Handle updates to that thread
    use_effect(move || {
        if *is_checking.read() || *is_downloading.read() {
            spawn(async move {
                while *is_checking.read() || *is_downloading.read() {
                    if let Some(receiver) = update_receiver.read().as_ref() {
                        while let Ok(update) = receiver.try_recv() {
                            match update {
                                UpdateStatus::UpdateAvailable(asset_name) => {
                                    pending_update.set(Some(asset_name));
                                    show_confirm_popup.set(true);
                                    is_checking.set(false);
                                }
                                UpdateStatus::NoUpdate => {
                                    is_checking.set(false);
                                }
                                UpdateStatus::DownloadComplete => {
                                    info.set("Update complete! Please re-launch the app.".to_string());
                                    is_downloading.set(false);
                                    showing_info.set(true);
                                    update_complete.set(true);
                                }
                                UpdateStatus::Failed(err) => {
                                    info.set(err);
                                    is_checking.set(false);
                                    is_downloading.set(false);
                                    showing_info.set(true);
                                }
                            }
                        }
                    }

                    // Don't freeze
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
            });
        }
    });

    // Handlers
    let handle_yes = move |_| {
        show_confirm_popup.set(false);
        
        if let Some(asset_name) = pending_update.read().clone() {
            is_downloading.set(true);
            
            let (sender, receiver) = mpsc::channel();
            update_receiver.set(Some(receiver));
            
            std::thread::spawn(move || {
                match perform_update(&asset_name) {
                    Ok(_) => {
                        let _ = sender.send(UpdateStatus::DownloadComplete);
                    }
                    Err(e) => {
                        let _ = sender.send(UpdateStatus::Failed(format!("Update failed: {}", e)));
                    }
                }
            });
        }
    };

    let handle_no = move |_| {
        show_confirm_popup.set(false);
        pending_update.set(None);
    };

    let window = dioxus::desktop::use_window();

    rsx! {
        if *show_confirm_popup.read() {
            div {
                class: "popup-overlay",
                div {
                    class: "popup-content",
                    h3 { "An update is available. Would you like to update now?" }
                    div {
                        style: "display: flex; gap: 10px; justify-content: center; margin-top: 20px;",
                        button {
                            class: "btn extract-btn",
                            onclick: handle_yes,
                            "Yes"
                        }
                        button {
                            class: "btn extract-btn",
                            onclick: handle_no,
                            "No"
                        }
                    }
                }
            }
        }

        if *is_downloading.read() {
            div {
                class: "popup-overlay",
                div {
                    class: "popup-content",
                    h3 { "Downloading update ..." }
                }
            }
        }

        if *showing_info.read() {
            div {
                class: "popup-overlay",
                div {
                    class: "popup-content",
                    h3 { "{*info.read()}" }
                    button {
                        class: "btn extract-btn",
                        onclick: move |_| {
                            showing_info.set(false);
                            if *update_complete.read() {
                                window.close();
                            }
                        },
                        "Close app"
                    }
                }
            }
        }
    }
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
                "SSGZ Version {CURRENT_VERSION}"
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

#[derive(Debug)]
enum UpdateStatus {
    UpdateAvailable(String),
    NoUpdate,
    DownloadComplete,
    Failed(String),
}