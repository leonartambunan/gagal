use dioxus::prelude::*;

pub fn Footer() -> Element {
    let categories = [
        (
            "OCR",
            vec![
                ("EKTP", "/ektp"),
                ("NPWP", "https://www.gagal.works/npwp"),
                ("SIM", "https://www.gagal.works/sim"),
            ],
        ),
        (
            "Tools",
            vec![
                ("Dev Tools", "/awesome"),
                ("Sec Tools", "/security"),
                
            ],
        ),
        (
            "AI",
            vec![
                ("Huggingface Candle", "https://github.com/huggingface/candle"),
                ("Kalosm", "https://github.com/floneum/floneum"),            
                ("Ollama", "https://github.com/ollama/ollama"),
            ],
        ),
    ];

    rsx! {
        footer { class: "text-gray-700 dark:text-gray-400 w-full mx-auto max-w-screen-xl px-2",
            div { class: "py-8 md:py-24 flex flex-wrap justify-evenly items-start lg:items-start md:flex-row md:flex-nowrap  gap-x-24 gap-y-8 mx-auto",
                for (name , links) in categories.iter() {
                    div { key: "{name}",
                        h2 { class: "text-md mb-3 text-black dark:text-gray-100", "{name}" }
                        nav { class: "list-none font-extralight ",
                            ul { class: "space-y-2",
                                for f in links.iter() {
                                    li { key: "{f.0}",
                                        a { class: "", href: "{f.1}", "{f.0}" }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "text-left md:text-left",
                    a {
                        class: "flex items-center gap-1",
                        href: "https://gagal.works",
                        div {
                            span { class: "text-lg font-mono dark:text-gray-100", "Gagal.WORKS" }
                        }
                        // img {
                        //     src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                        //     class: "h-6 w-auto",
                        //     alt: "Gagal WORKS Icon",
                        // }
                        img {
                            src: asset!("/assets/static/smalllogo.png"),
                            class: "h-6 w-auto",
                            alt: "Gagal means IT WILL WORKS",
                        }
                    }
                    span { class: "text-xs", "Gagal means IT WILL WORKS✌️" }
                }

            }
            div { class: "text-gray-400 text-sm text-center sm:text-left pb-2 mx-auto",
                "© 2025 Gagal.WORKS"
            }
        }
    }
}
