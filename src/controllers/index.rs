use actix_web::{HttpResponse, web, HttpRequest};

pub struct Index{}

impl Index {
    pub async fn test_index()-> &'static str {
        "test index page"
    }

    pub async fn index() {}
}
