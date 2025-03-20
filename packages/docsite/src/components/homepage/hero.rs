use crate::*;

pub(crate) fn Hero() -> Element {
    rsx! {
        section { class: "w-full mx-auto dark:text-white flex flex-col justify-between items-center  border-b  border-gray-300 min-h-[400px] flex-1 dark:border-[#a4a9ac7d] max-h-[400px] px-4",
            div { class: "flex w-full max-w-screen-xl flex-col text-center md:min-h-[400px] min-h-[400px] h-[calc(100vh-4rem)] gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    div { class: "text-center lg:text-left lg:flex-1",
                        div { class: "text-[2.5em] md:text-[3.5em] font-semibold dark:text-white text-ghdarkmetal font-sans leading-snug text-balance",
                            span { "Gagal." }
                            span { " WORKS" }
                        }
                        h3 { class: "text-[1.25em] dark:text-white font-light text-ghdarkmetal max-w-screen-sm md:max-w-screen-md md:text-left text-center flex flex-col",
                            span { class: "max-w-screen-md leading-loose",
                                "Gagal means it "
                                em { "IT WILL WORKS " }
                                "eventually."
                            }
                        }
                        div { class: "pt-8 lg:pt-16 text-[1em] flex flex-row space-x-4 mx-auto lg:mx-0 justify-center lg:justify-start",
                            
                        }
                    }
                    div { class: "lg:pb-12 h-screen max-h-40 lg:max-h-80 my-8",
                        img {
                            src: asset!("/assets/static/favicon.png"),
                            class: "dark:hidden w-full h-full",
                            alt: "Animated Icon",
                        }
                        img {
                            src: asset!("/assets/static/favicon.png"),
                            class: "hidden dark:block w-full h-full",
                            alt: "Animated Icon",
                        }
                    }
                }
            }


             
        }

        section { class: "w-full mx-auto dark:text-white flex flex-row justify-between items-center  border-b  border-gray-300 min-h-[200px] flex-1 dark:border-[#a4a9ac7d] max-h-[200px] px-4",
        div { class: "flex w-full max-w-screen-xl flex-row text-center md:min-h-[200px] min-h-[200px]  gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    span { "" }                    
                }   
            }
        }
        
    }      
}
