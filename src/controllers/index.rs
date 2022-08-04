use actix_web::{HttpResponse, web, HttpRequest};

use macros::tmpl::{Tpl};

pub struct Index{}

impl Index {
    pub async fn test_index()-> &'static str {
        "test index page"
    }

    pub async fn index(tpl: Tpl) -> HttpResponse {
        render!(tpl, "index.html")
    }
}
