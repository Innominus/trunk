use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    components::{A, FlatRoutes, Route, Router},
    lazy_route, Lazy, LazyRoute, StaticSegment,
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
                    <A attr:id="nav-analytics" href="/analytics">
                        "Analytics"
                    </A>
                    <A attr:id="nav-billing" href="/billing">
                        "Billing"
                    </A>
                    <A attr:id="nav-gallery" href="/gallery">
                        "Gallery"
                    </A>
                    <A attr:id="nav-inventory" href="/inventory">
                        "Inventory"
                    </A>
                    <A attr:id="nav-nested" href="/nested">
                        "Nested"
                    </A>
                    <A attr:id="nav-profile" href="/profile">
                        "Profile"
                    </A>
                    <A attr:id="nav-reports" href="/reports">
                        "Reports"
                    </A>
                    <A attr:id="nav-search" href="/search">
                        "Search"
                    </A>
                    <A attr:id="nav-settings" href="/settings">
                        "Settings"
                    </A>
                    <A attr:id="nav-team" href="/team">
                        "Team"
                    </A>
                    <span id="routing-state">
                        {move || if is_routing.get() { "Routing..." } else { "Idle" }}
                    </span>
                </nav>

                <FlatRoutes fallback=|| view! { <p id="not-found">"Not found."</p> }>
                    <Route path=StaticSegment("") view=Home />
                    <Route path=StaticSegment("about") view={Lazy::<AboutRoute>::new()} />
                    <Route path=StaticSegment("analytics") view={Lazy::<AnalyticsRoute>::new()} />
                    <Route path=StaticSegment("billing") view={Lazy::<BillingRoute>::new()} />
                    <Route path=StaticSegment("gallery") view={Lazy::<GalleryRoute>::new()} />
                    <Route path=StaticSegment("inventory") view={Lazy::<InventoryRoute>::new()} />
                    <Route path=StaticSegment("nested") view={Lazy::<NestedRoute>::new()} />
                    <Route path=StaticSegment("profile") view={Lazy::<ProfileRoute>::new()} />
                    <Route path=StaticSegment("reports") view={Lazy::<ReportsRoute>::new()} />
                    <Route path=StaticSegment("search") view={Lazy::<SearchRoute>::new()} />
                    <Route path=StaticSegment("settings") view={Lazy::<SettingsRoute>::new()} />
                    <Route path=StaticSegment("team") view={Lazy::<TeamRoute>::new()} />
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
            message.set("Loading lazy function...".to_string());
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
                "Load lazy greeting"
            </button>
        </section>
    }
}

#[leptos::lazy]
async fn load_greeting() -> String {
    "Hello from a Leptos lazy function.".to_string()
}

#[leptos::lazy(shared_metrics_fragment)]
async fn load_shared_metrics_fragment() -> String {
    "shared metrics fragment".to_string()
}

#[leptos::lazy(analytics_summary)]
async fn load_analytics_summary() -> String {
    let fragment = load_shared_metrics_fragment().await;
    format!("analytics summary -> {fragment}")
}

#[leptos::lazy(analytics_breakdown)]
async fn load_analytics_breakdown() -> String {
    let fragment = load_shared_metrics_fragment().await;
    format!("analytics breakdown -> {fragment}")
}

#[leptos::lazy(billing_statement)]
async fn load_billing_statement() -> String {
    let fragment = load_shared_metrics_fragment().await;
    format!("billing statement -> {fragment}")
}

#[leptos::lazy(billing_receipt)]
async fn load_billing_receipt() -> String {
    let fragment = load_shared_metrics_fragment().await;
    format!("billing receipt -> {fragment}")
}

#[leptos::lazy(gallery_snapshot)]
async fn load_gallery_snapshot() -> String {
    "gallery snapshot loaded from a named lazy function".to_string()
}

#[leptos::lazy(inventory_refresh)]
async fn load_inventory_refresh() -> String {
    "inventory refresh loaded from a named lazy function".to_string()
}

#[leptos::lazy(nested_inner)]
async fn load_nested_inner() -> String {
    "inner lazy function result".to_string()
}

#[leptos::lazy(nested_outer)]
async fn load_nested_outer() -> String {
    let inner = load_nested_inner().await;
    format!("outer lazy function result -> {inner}")
}

#[leptos::lazy(profile_badge)]
async fn load_profile_badge() -> String {
    "profile badge loaded from a named lazy function".to_string()
}

#[leptos::lazy(reports_digest)]
async fn load_reports_digest() -> String {
    "reports digest loaded from a named lazy function".to_string()
}

#[leptos::lazy(search_tokens)]
async fn load_search_tokens() -> String {
    "search token cache".to_string()
}

#[leptos::lazy(search_preview)]
async fn load_search_preview() -> String {
    let tokens = load_search_tokens().await;
    format!("search preview -> {tokens}")
}

#[leptos::lazy(search_clusters)]
async fn load_search_clusters() -> String {
    let tokens = load_search_tokens().await;
    format!("search clusters -> {tokens}")
}

#[leptos::lazy(settings_token)]
async fn load_settings_token() -> String {
    "settings token loaded from a named lazy function".to_string()
}

#[leptos::lazy(team_roster)]
async fn load_team_roster() -> String {
    "team roster loaded from a named lazy function".to_string()
}

#[leptos::lazy(team_badges)]
async fn load_team_badges() -> String {
    "team badges loaded from a named lazy function".to_string()
}

#[derive(Clone)]
struct AboutRoute;

#[lazy_route]
impl LazyRoute for AboutRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! {
            <section>
                <h1 id="page-title">"About"</h1>
                <p id="about-copy">"Loaded from a Leptos lazy route."</p>
                <A attr:id="back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct AnalyticsRoute;

#[lazy_route]
impl LazyRoute for AnalyticsRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_summary = move |_| {
            message.set("Loading analytics summary...".to_string());
            spawn_local(async move {
                message.set(load_analytics_summary().await);
            });
        };
        let load_breakdown = move |_| {
            message.set("Loading analytics breakdown...".to_string());
            spawn_local(async move {
                message.set(load_analytics_breakdown().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Analytics"</h1>
                <p id="analytics-copy">
                    "This lazy route owns two named lazy functions that share another lazy helper."
                </p>
                <p id="analytics-message">{move || message.get()}</p>
                <button id="analytics-summary-load" on:click=load_summary>
                    "Load analytics summary"
                </button>
                <button id="analytics-breakdown-load" on:click=load_breakdown>
                    "Load analytics breakdown"
                </button>
                <A attr:id="analytics-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct BillingRoute;

#[lazy_route]
impl LazyRoute for BillingRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_statement = move |_| {
            message.set("Loading billing statement...".to_string());
            spawn_local(async move {
                message.set(load_billing_statement().await);
            });
        };
        let load_receipt = move |_| {
            message.set("Loading billing receipt...".to_string());
            spawn_local(async move {
                message.set(load_billing_receipt().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Billing"</h1>
                <p id="billing-copy">
                    "This lazy route reuses the same shared lazy helper as analytics."
                </p>
                <p id="billing-message">{move || message.get()}</p>
                <button id="billing-statement-load" on:click=load_statement>
                    "Load billing statement"
                </button>
                <button id="billing-receipt-load" on:click=load_receipt>
                    "Load billing receipt"
                </button>
                <A attr:id="billing-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct GalleryRoute;

#[lazy_route]
impl LazyRoute for GalleryRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_snapshot = move |_| {
            message.set("Loading gallery snapshot...".to_string());
            spawn_local(async move {
                message.set(load_gallery_snapshot().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Gallery"</h1>
                <p id="gallery-copy">"This lazy route owns a named lazy function."</p>
                <p id="gallery-message">{move || message.get()}</p>
                <button id="gallery-load" on:click=load_snapshot>
                    "Load gallery snapshot"
                </button>
                <A attr:id="gallery-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct InventoryRoute {
    items: LocalResource<Vec<String>>,
}

#[lazy_route]
impl LazyRoute for InventoryRoute {
    fn data() -> Self {
        #[leptos::lazy(inventory_seed)]
        async fn load_inventory_seed() -> Vec<String> {
            vec![
                "sprocket".to_string(),
                "ratchet".to_string(),
                "adapter".to_string(),
            ]
        }

        Self {
            items: LocalResource::new(load_inventory_seed),
        }
    }

    fn view(this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_refresh = move |_| {
            message.set("Loading inventory refresh...".to_string());
            spawn_local(async move {
                message.set(load_inventory_refresh().await);
            });
        };

        let items = move || {
            Suspend::new(async move { this.items.await.join(", ") })
        };

        view! {
            <section>
                <h1 id="page-title">"Inventory"</h1>
                <p id="inventory-copy">
                    "This lazy route loads a named lazy function from route data and another from a button."
                </p>
                <p id="inventory-items">
                    <Suspense fallback=|| {
                        view! { <span id="inventory-loading">"Loading inventory seed..."</span> }
                    }>{items}</Suspense>
                </p>
                <p id="inventory-message">{move || message.get()}</p>
                <button id="inventory-refresh-load" on:click=load_refresh>
                    "Load inventory refresh"
                </button>
                <A attr:id="inventory-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct NestedRoute;

#[lazy_route]
impl LazyRoute for NestedRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_chain = move |_| {
            message.set("Loading nested lazy chain...".to_string());
            spawn_local(async move {
                message.set(load_nested_outer().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Nested"</h1>
                <p id="nested-copy">
                    "This lazy route calls a named lazy function that calls another named lazy function."
                </p>
                <p id="nested-message">{move || message.get()}</p>
                <button id="nested-load" on:click=load_chain>
                    "Load nested lazy chain"
                </button>
                <A attr:id="nested-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct ProfileRoute;

#[lazy_route]
impl LazyRoute for ProfileRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_badge = move |_| {
            message.set("Loading profile badge...".to_string());
            spawn_local(async move {
                message.set(load_profile_badge().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Profile"</h1>
                <p id="profile-copy">"This lazy route owns a named lazy function."</p>
                <p id="profile-message">{move || message.get()}</p>
                <button id="profile-load" on:click=load_badge>
                    "Load profile badge"
                </button>
                <A attr:id="profile-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct ReportsRoute;

#[lazy_route]
impl LazyRoute for ReportsRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_digest = move |_| {
            message.set("Loading reports digest...".to_string());
            spawn_local(async move {
                message.set(load_reports_digest().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Reports"</h1>
                <p id="reports-copy">"This lazy route owns a named lazy function."</p>
                <p id="reports-message">{move || message.get()}</p>
                <button id="reports-load" on:click=load_digest>
                    "Load reports digest"
                </button>
                <A attr:id="reports-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct SearchRoute;

#[lazy_route]
impl LazyRoute for SearchRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_preview = move |_| {
            message.set("Loading search preview...".to_string());
            spawn_local(async move {
                message.set(load_search_preview().await);
            });
        };
        let load_clusters = move |_| {
            message.set("Loading search clusters...".to_string());
            spawn_local(async move {
                message.set(load_search_clusters().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Search"</h1>
                <p id="search-copy">
                    "This lazy route owns two named lazy functions that both call a shared nested lazy helper."
                </p>
                <p id="search-message">{move || message.get()}</p>
                <button id="search-preview-load" on:click=load_preview>
                    "Load search preview"
                </button>
                <button id="search-clusters-load" on:click=load_clusters>
                    "Load search clusters"
                </button>
                <A attr:id="search-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct SettingsRoute;

#[lazy_route]
impl LazyRoute for SettingsRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_token = move |_| {
            message.set("Loading settings token...".to_string());
            spawn_local(async move {
                message.set(load_settings_token().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Settings"</h1>
                <p id="settings-copy">"This lazy route owns a named lazy function."</p>
                <p id="settings-message">{move || message.get()}</p>
                <button id="settings-load" on:click=load_token>
                    "Load settings token"
                </button>
                <A attr:id="settings-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

#[derive(Clone)]
struct TeamRoute;

#[lazy_route]
impl LazyRoute for TeamRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let message = RwSignal::new("Not loaded yet.".to_string());
        let load_roster = move |_| {
            message.set("Loading team roster...".to_string());
            spawn_local(async move {
                message.set(load_team_roster().await);
            });
        };
        let load_badges = move |_| {
            message.set("Loading team badges...".to_string());
            spawn_local(async move {
                message.set(load_team_badges().await);
            });
        };

        view! {
            <section>
                <h1 id="page-title">"Team"</h1>
                <p id="team-copy">"This lazy route owns two sibling named lazy functions."</p>
                <p id="team-message">{move || message.get()}</p>
                <button id="team-roster-load" on:click=load_roster>
                    "Load team roster"
                </button>
                <button id="team-badges-load" on:click=load_badges>
                    "Load team badges"
                </button>
                <A attr:id="team-back-home" href="/">
                    "Back home"
                </A>
            </section>
        }
        .into_any()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
