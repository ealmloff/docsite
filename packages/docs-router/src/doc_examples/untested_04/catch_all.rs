#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: router
#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
// ANCHOR_END: router

// ANCHOR: app
#[component]
fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
// ANCHOR_END: app

// ANCHOR: home
#[component]
fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}
// ANCHOR_END: home

// ANCHOR: fallback
#[component]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
// ANCHOR_END: fallback

fn main() {}
