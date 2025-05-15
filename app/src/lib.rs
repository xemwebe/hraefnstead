use hraefnstead_lib::{parser::parse, state::State};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

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
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = command_input.get().expect("<command> to exist").value();

        let new_command = parse(&value);
        if let Some(command_stack) = state.write().special_event_triggered(&new_command) {
            for new_command in command_stack {
                state.update(|s| {
                    let _ = new_command.execute(s);
                });
            }
        } else {
            state.update(|s| {
                let _ = new_command.execute(s);
            });
        }
        let mut msg = String::new();
        state.update(|s| msg = s.get_log());
        output.set(format!("{}\n{}\n\n---> {}\n", output.get(), msg, value));
        command_input
            .get()
            .expect("<command> to exist")
            .set_value("");
    };

    view! {
        <h1>"Hraefnstead - a tiny text adventure"</h1>
            <div>
            <textarea class="scrollabletextbox" name="note" readonly prop:value=move || output.get()>{output}</textarea>
            { output }
            </div>
            <div class="command">
            <p>"Enter your command: "</p>
            <form on:submit=on_submit>
                <input class="command_input" type="text" value=output node_ref=command_input />
                <input type="submit" value="Submit"/>
            </form>
            <button on:click=move |_| { state.set(State::new()); }>"Restart game"</button>
            </div>
    }
}
