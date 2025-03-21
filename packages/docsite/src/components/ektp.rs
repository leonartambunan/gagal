use crate::*;

pub(crate) mod ektp_hero;

#[component]
pub(crate) fn EKTP() -> Element {
    rsx! {
        div { class: "w-full",
            ektp_hero::EKTPHero{}
        }
    }
}

#[component]
fn EktpTriShow(
    left: Element,
    center: Element,
    right: Element,
    title: &'static str,
    to: Route,
    last: Option<bool>,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-row justify-center max-w-screen-lg",
            EktpTriPadding { last: last.unwrap_or_default(), {center} }
            div { class: "grow basis-0",
                Link { to: to.clone(),
                    div { class: "min-w-lg max-w-screen-md hover:shadow-pop rounded-lg p-8",
                        h2 { class: "text-2xl text-gray-800 font-semibold pb-2 dark:text-gray-100 ",
                            "{title}"
                        }
                        {right}
                    }
                }
            }
        }
    }
}

#[component]
fn EktpTriPadding(children: Element, last: bool) -> Element {
    rsx!(
        div { class: "flex flex-col items-center",
            div { class: "w-0 h-10 border-dashed border border-[#444]" }
            EktpIconSplit {}
            if !last {
                div { class: "w-0 h-full border-dashed border border-[#444]", {children} }
            }
        }
    )
}

#[component]
fn EktpExperienceText(title: &'static str, content: &'static str) -> Element {
    rsx!(
        div { class: "pb-12",
            h3 { class: "text-2xl text-gray-800 font-semibold pb-2 dark:text-gray-100 ",
                "{title}"
            }
            p { "{content}" }
        }
    )
}

 fn EktpIconSplit() -> Element {
     rsx! {
         svg {
             class: "mx-auto fill-[#444] dark:fill-white",
             version: "1.1",
             view_box: "0 0 24 24",
             width: "24",
             "data-view-component": "true",
             "aria-hidden": "true",
             height: "24",
             path {
                 stroke_width: "1.5",
                 fill_rule: "evenodd",
                 d: "M15.5 11.75a3.5 3.5 0 11-7 0 3.5 3.5 0 017 0zm1.444-.75a5.001 5.001 0 00-9.888 0H2.75a.75.75 0 100 1.5h4.306a5.001 5.001 0 009.888 0h4.306a.75.75 0 100-1.5h-4.306z",
             }
         }
     }
 }


#[component]
fn EktpStatsItem(major: String, minor: String) -> Element {
    rsx! {
        div { class: "text-center shadow mx-2 rounded-lg py-6 border",
            div { class: "text-5xl font-bold text-gray-800 dark:text-gray-100", {major} }
            div { class: "text-xl text-gray-600 dark:text-gray-400", {minor} }
        }
    }
}
