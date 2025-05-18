use hraefnstead_lib::{command::Command, parser::parse, state::State, victory::Victory, GAME_OVER};
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

fn process_victory(state: &mut State, victory: &Victory) {
    match victory {
        Victory::Won => {
            state.log("\n!!!Congratulations You won the Game!!!");
        }
        Victory::GameOver => {
            state.log("\nYou are dead.\nIf you want to try again, press the restart button or load a previously saved state.");
            state.set_location(GAME_OVER);
        }
        Victory::Load(_) | Victory::Save(_) => {
            state.log("\nPlease use the button below to save or load game states.")
        }
        Victory::Quit => {
            state.log("\nIf you want to quit, just close the window.");
        }
        Victory::None => {}
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use leptos::ev::SubmitEvent;
    use leptos::html::Input;

    let state = RwSignal::new(State::new());
    // Creates a reactive value to update the button
    let output = RwSignal::new("Welcome to the dungeons of hraefnstead!\nType:'help' to briefly view possible actions.\nTyping said actions prior to 'help' will reveal more about their quality.\n".to_string());
    let command_input: NodeRef<Input> = NodeRef::new();
    let output_area: NodeRef<Textarea> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = command_input.get().expect("<command> to exist").value();

        let mut new_command = Command::None;
        state.update(|s| {
            new_command = parse(&value, s);
        });
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
        state.update(|s| process_victory(s, &command_result));
        let mut log = String::new();
        state.update(|s| log = s.get_log());
        output.set(format!("---> {value}\n{log}\n"));
        command_input
            .get()
            .expect("<command> to exist")
            .set_value("");

        if let Some(o) = output_area.get() {
            o.set_scroll_top(o.scroll_height());
        }
    };

    view! {
        <h1>"Hraefnstead - a tiny text adventure"</h1>
            <div>
            <textarea class="scrollabletextbox" name="note" readonly prop:value=move || output.get() node_ref=output_area ></textarea>
            </div>
            <div class="command">
            <p>"Enter your command: "</p>
            <form on:submit=on_submit>
                <input class="command_input" type="text" value="" node_ref=command_input />
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
