use leptos::*;
use leptos_router::*;
use leptos_use::{use_websocket, UseWebSocketReadyState, UseWebsocketReturn};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="root">
            <Router>
                <h1>Leptos-use Error on Routing</h1>
                <A href="">Home</A>
            <main>
            <Routes>
                // TODO - new game & join game suspense
                <Route path="" view=|cx| view!{cx, <A href="test">Start websocket</A>} />
                <Route path="/:id" view=|cx| view!{ cx,
                    <WebsocketPage />
                } />
            </Routes>
            </main>
            </Router>
        </div>
    }
}
#[component]
pub fn WebsocketPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let (display_msg, set_display_msg) = create_signal::<Option<String>>(cx, None);

    // TODO - use_websocket causes panic on route change - investigate
    let UseWebsocketReturn {
        ready_state,
        message,
        send,
        ..
    } = use_websocket(cx, "ws://127.0.0.1:3000/api/websocket".to_string());

    create_effect(cx, move |_| {
        if ready_state() == UseWebSocketReadyState::Open {
            send(id());
        }
    });

    create_effect(cx, move |_| {
        if let Some(msg) = message() {
            set_display_msg(Some(msg));
        }
    });

    view! { cx,
        <div class="error">{display_msg}</div>
    }
}
