use crate::*;

pub(crate) fn OCRHero() -> Element {
    rsx! {
        section { class: "w-full mx-auto dark:text-white flex flex-col justify-between items-center  border-b  border-gray-300 min-h-[760px] flex-1 dark:border-[#a4a9ac7d] max-h-[860px] px-4",
            div { class: "flex w-full max-w-screen-xl flex-col text-center md:min-h-[520px] min-h-[560px] h-[calc(100vh-4rem)] gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
                    
                Link { to: NavigationTarget::<Route>::Internal(Route::EKTP {}), new_tab: false,
                    div { class: "flex flex-col h-full p-3 rounded hover:-translate-y-2 transition-transform duration-300 shadow border border-gray-800",
                        div {
                            p { class: "text-xl text-gray-800 dark:text-gray-100 font-bold",
                                "EKTP"
                            }
                            p { class: "text-base pt-2 text-gray-700 dark:text-gray-400",
                                "Indonesian Identification Card"
                            }
                        }
                    }
                }                            

                Link { to: NavigationTarget::<Route>::Internal(Route::SIM{}), new_tab: false,
                    div { class: "flex flex-col h-full p-3 rounded hover:-translate-y-2 transition-transform duration-300 shadow border border-gray-800",
                        div {
                            p { class: "text-xl text-gray-800 dark:text-gray-100 font-bold",
                                "SIM"
                            }
                            p { class: "text-base pt-2 text-gray-700 dark:text-gray-400",
                                "Indonesian Driving Licence Card"
                            }
                        }
                    }
                }     

                Link { to: NavigationTarget::<Route>::Internal(Route::NPWP {  }), new_tab: false,
                div { class: "flex flex-col h-full p-3 rounded hover:-translate-y-2 transition-transform duration-300 shadow border border-gray-800",
                    div {
                        p { class: "text-xl text-gray-800 dark:text-gray-100 font-bold",
                            "NPWP"
                        }
                        p { class: "text-base pt-2 text-gray-700 dark:text-gray-400",
                            "Indonesian TAX ID Card"
                        }
                    }
                }
            }     

            }
        }
    }
}
}

