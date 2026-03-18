use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    components::{A, FlatRoutes, Route, Router},
    StaticSegment,
};

#[component]
fn App() -> impl IntoView {
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Router set_is_routing>
            <main>
                <nav>
                    <A attr:id="nav-home" href="/">
                        "Home"
                    </A>
                    <A attr:id="nav-about" href="/about">
                        "About"
                    </A>
                    <span id="routing-state">
                        {move || if is_routing.get() { "Routing..." } else { "Idle" }}
                    </span>
                </nav>

                <FlatRoutes fallback=|| view! { <p id="not-found">"Not found."</p> }>
                    <Route path=StaticSegment("") view=Home />
                    <Route path=StaticSegment("about") view=About />
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let message = RwSignal::new("Not loaded yet.".to_string());

    let set_local = {
        move |_| {
            message.set("Updated from the main bundle.".to_string());
        }
    };

    let set_lazy = {
        move |_| {
            message.set("Loading eager function...".to_string());
            spawn_local(async move {
                message.set(load_greeting().await);
            });
        }
    };

    view! {
        <section>
            <h1 id="page-title">"Home"</h1>
            <p id="home-copy">"This route ships in the main bundle."</p>
            <p id="lazy-message">{move || message.get()}</p>
            <button id="local-update" on:click=set_local>
                "Update locally"
            </button>
            <button id="lazy-update" on:click=set_lazy>
                "Load greeting"
            </button>
        </section>
    }
}

async fn load_greeting() -> String {
    "Hello from the eager baseline function.".to_string()
}

#[component]
fn About() -> impl IntoView {
    view! {
        <section>
            <h1 id="page-title">"About"</h1>
            <p id="about-copy">"Loaded from the eager main bundle route."</p>
            <A attr:id="back-home" href="/">
                "Back home"
            </A>
        </section>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
