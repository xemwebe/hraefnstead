use hraefnstead_lib::{parser::parse, state::State, victory::Victory};
use leptos::{html::Textarea, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

mod files;

use files::{FileDownload, FileUpload};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/hraefnstead.css"/>

        // sets the document title
        <Title text="Hraefnstead"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use leptos::ev::SubmitEvent;
    use leptos::html::Input;

    let state = RwSignal::new(State::new());
    // Creates a reactive value to update the button
    let output = RwSignal::new(String::new());
    let command_input: NodeRef<Input> = NodeRef::new();
    let output_area: NodeRef<Textarea> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = command_input.get().expect("<command> to exist").value();

        let new_command = parse(&value);
        let mut command_result = Victory::None;
        let mut command_stack = None;
        state.update(|s| command_stack = s.special_event_triggered(&new_command));
        if let Some(command_stack) = command_stack {
            for new_command in command_stack {
                state.update(|s| {
                    command_result = new_command.execute(s);
                });
            }
        } else {
            state.update(|s| {
                command_result = new_command.execute(s);
            });
        }
        let mut log = String::new();
        state.update(|s| log = s.get_log());
        output.set(format!("{}\n---> {value}\n{log}\n", output.get()));
        command_input
            .get()
            .expect("<command> to exist")
            .set_value("");

        output_area.update(|o| {
            if let Some(o) = o {
                o.set_scroll_top(o.scroll_height());
            }
        });
    };

    view! {
        <h1>"Hraefnstead - a tiny text adventure"</h1>
            <div>
            <textarea class="scrollabletextbox" name="note" readonly prop:value=move || output.get() ></textarea>
            </div>
            <div class="command">
            <p>"Enter your command: "</p>
            <form on:submit=on_submit>
                <input class="command_input" type="text" value=output node_ref=command_input />
                <input type="submit" value="Submit"/>
            </form>
            </div>
            <div class="buttons">
            <button on:click=move |_|  {
                state.set(State::new());
                output.set(String::new()); }
            >"Restart game"</button>
            <FileUpload state />
            <FileDownload state />
            </div>
    }
}
