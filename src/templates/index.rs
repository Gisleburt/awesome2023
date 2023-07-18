use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
            h1 { "Awesome '23!" }
            p {
                "Join me and a bunch of awesome folks I know for drinks!"
            }
            p {
                "On "
                strong { "August 12th" }
                " at the "
                a(href = "https://clerkenwellandsocial.com/#footer") { "Clerkenwell and Social" }
                " from 6:00pm"
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Awesome '23!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
