#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod icons;
pub mod shortcut;
pub use components::*;
pub mod ai_client;

fn main() {
    // If we are just building the search index, we don't need to launch the app
    #[cfg(feature = "server")]
    if std::env::args().any(|arg| arg == "--generate-search-index") {
        search::generate_search_index();
        return;
    }

    create_sitemap();

    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            // Only in release do we SSG
            let mut cfg = ServeConfig::builder();

            if !cfg!(debug_assertions) {
                cfg = cfg.incremental(
                    IncrementalRendererConfig::new()
                        .static_dir(static_dir())
                        .clear_cache(false)
                );
            }

            cfg.build().expect("Unable to build ServeConfig")
        })
        .launch(|| {
            rsx! {
                Router::<Route> {}
            }
        });
        
}

#[component]
fn HeaderFooter() -> Element {
    let cb = use_callback(|_| *SHOW_SEARCH.write() = true);

    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || cb.call(())
    });

    rsx! {
        Head {}
        div { class: "bg-white dark:bg-black min-h-screen",
            Nav {}
            div {
                Outlet::<Route> {}
                Footer {}
            }
        }
    }
}

fn Head() -> Element {
    use document::{Link, Meta, Script, Stylesheet, Title};

    // let current_doc_route = use_route::<Route>();

    rsx! {
        Title { "Gagal.Works | OCR, Tools, Blog" }
        Meta {
            name: "description",
            content: "Gagal.Works | OCR, Tools, Blog",
        }
        Link {
            rel: "icon shortcut",
            r#type: "image/png",
            href: asset!("/assets/static/favicon.png"),
        }

        Stylesheet { href: asset!("/assets/githubmarkdown.css") }
        Stylesheet { href: asset!("/assets/tailwind.css", CssAssetOptions::new().with_minify(false)) }
        Stylesheet { href: asset!("/assets/main.css") }
        Stylesheet { href: asset!("/assets/material.css") }
        Stylesheet { href: "https://rsms.me/inter/inter.css" }
        Stylesheet { href: asset!("/assets/fileupload.css") }

        Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        Link {
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
            crossorigin: "false",
        }
        Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Arimo:wght@100;400;600&display=swap",
        }
        Link {
            href: "https://fonts.googleapis.com/css2?family=Arimo:ital,wght@0,400..700;1,400..700&family=Lexend:wght@100;400&family=M+PLUS+1:wght@100..900&family=Poppins:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap",
            rel: "stylesheet",
        }
        Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
        }
        Meta {
            property: "og:title",
            content: "Gagal.WORKS | OCR, Rust, Tools",
        }
        Meta { property: "og:type", content: "website" }
        Meta {
            property: "og:description",
            content: "A toolbox web site.",
        }
        Meta { property: "og:url", content: "https://gagal.works" }
        Meta {
            property: "og:image",
            content: "https://dioxuslabs.com/assets/static/opengraph.png",
        }
        Meta {
            name: "twitter:title",
            content: "Gagal.WORKS | OCR, Rust, Tools",
        }
        Meta {
            name: "twitter:description",
            content: "Gagal.WORKS | OCR, Rust, Tools",
        }
        
        Script {
            r#async: true,
            src: "https://www.googletagmanager.com/gtag/js?id=G-EBE72MVZ1B",
        }
        Script {
            r#async: true,
            src: asset!("/assets/gtag.js"),
            r#type: "text/javascript",
        }
        // if don_t_index {
        //     Meta { name: "robots", content: "noindex" }
        // }
    }
}

#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(HeaderFooter)]
        #[route("/")]
        Homepage {},

        #[route("/awesome")]
        AwesomeRust {},

        #[route("/security")]
        AwesomeSec {},

        #[route("/ocrs")]
        OCRS {},

        #[route("/ektp")]
        EKTP {},

        #[route("/sim")]
        SIM {},

        #[route("/npwp")]
        NPWP {},

        // #[nest("/blog")]
        //     #[route("/")]
        //     BlogList {},
        //     #[layout(BlogPost)]
        //         #[child("")]
        //         BlogPost { child: crate::docs::router_blog::BookRoute },
        //     #[end_layout]
        // #[end_nest]

    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

impl Route {
}

#[cfg(feature = "fullstack")]
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}

fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

fn create_sitemap() {
    #[cfg(not(debug_assertions))]
    server_only! {
        use std::io::Write;

        // Write a sitemap file on the server
        // The sitemap helps with SEO because google will deprioritize pages it finds that are not in the sitemap
        let all_routes = Route::static_routes();
        _ = std::fs::create_dir_all(static_dir());
        let output_path = static_dir().join("sitemap.xml");
        let Ok(file) = std::fs::File::create(output_path) else {
            eprintln!("Failed to create sitemap file");
            return;
        };
        let mut writer = std::io::BufWriter::new(file);
        _ = writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        _ = writeln!(writer, r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        for route in all_routes {
            // If the documentation is out of date, don't include it in the sitemap
            if route.is_docs() && !route.is_latest_docs() {
                continue;
            }
            _ = writeln!(writer, r#"<url>"#);
            let url = format!("https://gagal.works{}", route);
            let escaped_url = askama_escape::escape(&url, askama_escape::Html);
            _ = writeln!(writer, r#"    <loc>{}</loc>"#, escaped_url);
            _ = writeln!(writer, r#"</url>"#);
        }
        _ = writeln!(writer, r#"</urlset>"#);

        // Point to the sitemap file in the robots.txt
        _ = std::fs::write(static_dir().join("robots.txt"), format!("Sitemap: https://gagal.works/sitemap.xml"));
    }
}
