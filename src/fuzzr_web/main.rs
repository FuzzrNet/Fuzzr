use warp::Filter;
// use maud::{ DOCTYPE, html, Markup, Render };
// use serde::{ Serialize, Deserialize };
extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub struct Stylesheet(&'static str);

impl Render for Stylesheet {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "fuzzr");
    }

    pretty_env_logger::init();



    fn header(page_title: &str) -> Markup {
        html! {
            (DOCTYPE)
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { (page_title) }
        }
    }

    pub fn page(title: &str) -> Markup {
        html! {
            // Add the header markup to the page
            (header(title))
            h1 { (title) }
        }
    }

    // Routes
    let static_web = warp::fs::dir("fuzzr_web_gui/");

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("static_web/index.html"));

    let favicon = warp::get()
        .and(warp::path::path("favicon.ico"))
        .and(warp::fs::file("static_web/favicon.ico"));

    let routes = index.or(favicon).or(static_web).or(download_route);

    // Serve routes
    info!("Serving fuzz...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
