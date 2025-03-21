use crate::*;

pub(crate) static SHOW_SEARCH: GlobalSignal<bool> = Signal::global(|| false);

pub(crate) fn Nav() -> Element {
    let _route: Route = use_route();

    rsx! {
        // header { class: "sticky top-0 z-30 bg-opacity-80 dark:text-gray-200 dark:bg-opacity-80 border-b border-stone-300 dark:border-stone-700 h-16 backdrop-blur-sm",
        header { class: "sticky top-0 z-30 bg-opacity-80 dark:text-gray-200 dark:bg-opacity-80 border-stone-300 dark:border-stone-700 h-16 backdrop-blur-sm",
          div { class: "py-2 px-2 max-w-screen-2xl mx-auto flex items-center justify-between text-sm leading-6 h-16",
            div { class: "flex z-50 md:px-2 flex-1", LinkList {} }
            div { class: "flex h-full justify-end ml-2 items-center gap-3 py-2",}
          }
        }
    }
}

static LINKS: &[(&str, &str)] = &[
    ("OCR", "/ocrs"),
    ("Dev Tools", "/dev"),
    ("Sec Tools", "/security"),
    ("AI Tools", "/ai"),
];

 #[component]
 fn LinkList() -> Element {
     rsx! {
         nav { class: "flex-grow md:flex-grow-0 flex flex-row items-center  text-md font-light leading-none text-slate-700 dark:text-white whitespace-nowrap md:gap-6",
             Link {
                 to: Route::Homepage {},
                 class: "title-font font-medium items-center text-gray-900 flex flex-row gap-1",
                 img {
                     src: asset!("/assets/static/smalllogo.png"),
                     class: "h-6 w-auto",
                 }
                 span { class: "text-xl dark:text-white leading-none hidden sm:block font-mono",
                     "Gagal.WORKS"
                 }
             }
             div { class: "flex-1 flex flex-row items-center md:space-x-6 justify-evenly",
                 for (name , link) in LINKS.iter().cloned() {
                     Link {
                         to: link,
                         class: "leading-none hover:text-sky-500 dark:hover:text-sky-400 rounded fill-zinc-700 dark:fill-zinc-100",
                         active_class: "text-sky-500 dark:text-sky-400",
                         position: "relative",
                         "{name}"
                     }
                 }
             }
         }
     }
 }
