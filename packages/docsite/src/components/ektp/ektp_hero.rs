use crate::{ai_client::ai_client::ask_ai, *};
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;

// use dioxus::prelude::dioxus_elements::FileEngine;
// use std::sync::Arc;

pub(crate) fn EKTPHero() -> Element {
    let mut hovered: Signal<bool> = use_signal(|| false);
    let mut final_result: Signal<String> = use_signal(|| String::new());

    // let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
    //     let files: Vec<String> = file_engine.files();
    //     for file_name in &files {
    //         if let Some(contents) = file_engine.read_file(file_name).await {
    //             let base_64_str: String = BASE64_STANDARD_NO_PAD.encode(contents);
    //             let ocr = ask_ai("ektp", base_64_str).await;
    //             println!("{ocr:?}");
    //             final_result.set(ocr.to_string());
    //         }
    //     }
    // };

    let upload_files = move |evt: FormEvent| async move {

        final_result.set("Processing...(in 1cpu and 1GBram; please be patient)".to_string());
        if let Some(file_engine) = evt.files() {

            let files: Vec<String> = file_engine.files();
            for file_name in &files {
                if let Some(contents) = file_engine.read_file(file_name).await {
                    let base_64_str: String = BASE64_STANDARD_NO_PAD.encode(contents);
                    let ocr = ask_ai("ektp", base_64_str).await;
                    println!("{ocr:?}");
                    final_result.set(ocr.to_string());
                }
            }

            //read_files(file_engine).await;
        }
    };

    rsx! {
        section { class: "w-full mx-auto dark:text-white flex flex-col justify-between items-center  border-b  border-gray-300 min-h-[760px] flex-1 dark:border-[#a4a9ac7d] max-h-[960px] px-4",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    div { class: "text-center lg:text-left lg:flex-1",
                        div { class: "text-[2.5em] md:text-[3.5em] font-semibold dark:text-white text-ghdarkmetal font-sans leading-snug text-balance",
                            span { "EKTP Indonesia" }
                        }
                        div { class: "pt-8 lg:pt-16 text-[1em] flex flex-row space-x-4 mx-auto lg:mx-0 justify-center lg:justify-start",                           

                    }

                h1 { "Upload Indonesian EKTP" }
                p { "Drop a .png, .jpg, or .jpeg file here to perform OCR on it" }
                    //button { onclick: move |_| files_uploaded.write().clear(), "Clear files" }
                div {
                    id: "drop-zone",
                    background_color: if hovered() { "lightblue" } else { "lightgray" },
                    label { r#for: "textreader", "::" }
                    input {
                        r#type: "file",
                        accept: ".png,.jpg,.jpeg",
                        multiple: false,
                        name: "textreader",
                        directory: false,
                        onchange: upload_files,
                        ondragover: move |evt| {
                            evt.prevent_default();
                            hovered.set(true)
                            },
                        ondragleave: move |_| hovered.set(false),
                    }
                }
                div {  
                    h3 { class: "text-[1.25em] dark:text-white font-light text-ghdarkmetal max-w-screen-sm md:max-w-screen-md md:text-left text-center flex flex-col",
                        span { class: "max-w-screen-md leading-loose",
                            "Result:"
                        }
                    }
                }
                
                div {
                    h1 { "" }
                        textarea {
                            id: "result",
                            rows: "20",
                            cols: "70",
                            placeholder: "OCR result here...",
                            value: "{final_result}",                                        
                        }
                    }
                }
            }
        }
    }
}
    