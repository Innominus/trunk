use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::spawn_local;
use wasm_split_helpers::wasm_split;
use web_sys::{Document, Element, Event, History, Window};

const HOME_ROUTE: &str = "/";
const DETAILS_ROUTE: &str = "/details";
const HOME_TITLE: &str = "Home";
const HOME_BODY: &str = "Main bundle is ready.";
const DETAILS_TITLE: &str = "Details";

#[wasm_split(lazy_counter)]
fn lazy_increment() {
    let next = current_count().unwrap_or(0) + 7;
    let _ = set_count(next);
    set_status("Lazy split chunk loaded.");
}

#[wasm_split(details_route)]
fn show_details_route() {
    let _ = set_text("route-title", DETAILS_TITLE);
    let _ = set_text("route-body", "Details route loaded from a split WASM chunk.");
    set_status("Details route chunk loaded.");
}

fn main() {
    if let Err(err) = start() {
        web_sys::console::error_1(&err);
    }
}

fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    install_handlers()?;

    spawn_local(async {
        report(render_route(current_path()).await);
    });

    Ok(())
}

fn install_handlers() -> Result<(), JsValue> {
    on_click("local-add", || {
        report(adjust_count(1));
        set_status("Local counter updated.");
    })?;

    on_click("lazy-add", || {
        set_status("Loading lazy split chunk...");
        spawn_local(async {
            lazy_increment().await;
        });
    })?;

    on_click("nav-home", || {
        report(navigate(HOME_ROUTE));
    })?;

    on_click("nav-details", || {
        report(navigate(DETAILS_ROUTE));
    })?;

    let on_popstate = Closure::wrap(Box::new(move |_event: Event| {
        spawn_local(async {
            report(render_route(current_path()).await);
        });
    }) as Box<dyn FnMut(_)>);
    window()?.add_event_listener_with_callback("popstate", on_popstate.as_ref().unchecked_ref())?;
    on_popstate.forget();

    Ok(())
}

fn navigate(path: &'static str) -> Result<(), JsValue> {
    history()?.push_state_with_url(&JsValue::NULL, "", Some(path))?;
    spawn_local(async move {
        report(render_route(path.to_string()).await);
    });
    Ok(())
}

async fn render_route(path: String) -> Result<(), JsValue> {
    match path.as_str() {
        DETAILS_ROUTE => {
            set_text("route-title", DETAILS_TITLE)?;
            set_text("route-body", "Loading route chunk...")?;
            set_status("Loading details route...");
            show_details_route().await;
        }
        _ => {
            set_text("route-title", HOME_TITLE)?;
            set_text("route-body", HOME_BODY)?;
            set_status("Home route ready.");
        }
    }

    Ok(())
}

fn on_click(id: &'static str, mut handler: impl FnMut() + 'static) -> Result<(), JsValue> {
    let closure = Closure::wrap(Box::new(move |_event: Event| {
        handler();
    }) as Box<dyn FnMut(_)>);
    element(id)?.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

fn adjust_count(delta: u32) -> Result<(), JsValue> {
    set_count(current_count()? + delta)
}

fn current_count() -> Result<u32, JsValue> {
    let text = element("count-value")
        .and_then(text_content)?
        .strip_prefix("Count: ")
        .ok_or_else(|| JsValue::from_str("count element is missing prefix"))?
        .parse::<u32>()
        .map_err(|err| JsValue::from_str(&format!("invalid count text: {err}")))?;
    Ok(text)
}

fn set_count(value: u32) -> Result<(), JsValue> {
    set_text("count-value", &format!("Count: {value}"))
}

fn set_status(message: &str) {
    report(set_text("status", message));
}

fn set_text(id: &str, value: &str) -> Result<(), JsValue> {
    element(id)?.set_text_content(Some(value));
    Ok(())
}

fn current_path() -> String {
    window()
        .and_then(|window| window.location().pathname().map_err(Into::into))
        .unwrap_or_else(|_| HOME_ROUTE.to_string())
}

fn element(id: &str) -> Result<Element, JsValue> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing element #{id}")))
}

fn text_content(element: Element) -> Result<String, JsValue> {
    element
        .text_content()
        .ok_or_else(|| JsValue::from_str("element is missing text content"))
}

fn document() -> Result<Document, JsValue> {
    window()?
        .document()
        .ok_or_else(|| JsValue::from_str("missing document"))
}

fn history() -> Result<History, JsValue> {
    window()?.history()
}

fn window() -> Result<Window, JsValue> {
    web_sys::window().ok_or_else(|| JsValue::from_str("missing window"))
}

fn report(result: Result<(), JsValue>) {
    if let Err(err) = result {
        web_sys::console::error_1(&err);
    }
}
