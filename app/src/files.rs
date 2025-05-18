use hraefnstead_lib::state::State;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::info;

#[component]
pub fn FileUpload(state: RwSignal<State>) -> impl IntoView {
    let file_selector: NodeRef<Input> = NodeRef::new();

    let load_file = move |_| {
        if let Some(files_input) = file_selector.get_untracked() {
            if let Some(files) = files_input.files() {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();
                    info!("loading file {file_name}");
                    spawn_local(async move {
                        if let Ok(file_content) =
                            wasm_bindgen_futures::JsFuture::from(file.text()).await
                        {
                            if let Some(file_content) = file_content.as_string() {
                                let new_state = serde_json::from_str(&file_content);
                                if let Ok(new_state) = new_state {
                                    state.set(new_state);
                                }
                            }
                        }
                    });
                }
            }
        }
    };

    view! {
         <span>
             <button class="custom-button" on:click=load_file >Load File</button>
             <input type="file" name="file_to_upload" id="file_to_upload" class="file-input" node_ref=file_selector />
         </span>
    }
}

#[component]
pub fn FileDownload(state: RwSignal<State>) -> impl IntoView {
    let download_file = move || {
        format!(
            "data:text/plain;charset=utf-8,{}",
            serde_json::to_string(&(state.get())).unwrap()
        )
    };

    view! {
        <a class="button" href=download_file download="adventure_state.json">Save Game</a>
    }
}
