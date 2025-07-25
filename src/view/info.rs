use crate::config::Server;
use crate::view::index;
use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get, web};
use maud::{Markup, html};

#[get("/info")]
pub async fn info_endpoint(server: web::Data<Server>, _req: HttpRequest) -> AwResult<Markup> {
    let should_poll_reload = server.db_token().is_none();
    Ok(index(Some(info()), should_poll_reload))
}

pub fn info() -> Markup {
    html! {
        div class="mx-4" {
            (info_card(
                "Chat",
                "Use the Rezi chat to talk about recipes",
                "Chat",
                ""
            ))
        }
    }
}

pub fn info_card(title: &str, description: &str, link_title: &str, link: &str) -> Markup {
    html! {
        div class="card w-96 bg-base-100 card-xs shadow-sm" {
            div class="card-body" {
                h2 class="card-title" {
                    (title)
                }
                p {
                    (description)
                }
                div class="justify-end card-actions" {
                    a class="btn btn-primary" href=(format!("/{}", link)) {
                        (link_title)
                    }
                }
            }
        }
    }
}
