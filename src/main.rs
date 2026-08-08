use dioxus::prelude::*;
mod components;
use components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

//static CSS: Asset = asset!("/assets/main.css");
static ICON: Asset = asset!("/assets/pfp.jpeg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        div { class:"min-h-screen bg-black text-white" ,
        div {
            class:"flex items-center gap-4",
            img {
                class: "size-108 shadow-xl rounded-full",
                src: ICON,
            }
            div{
            h1 {
                class:"text-4xl",
                "Cools9"
             }
            Accordion{allow_multiple_open: false, horizontal: false,
            AccordionItem { index: 0,
                    AccordionTrigger { "Who am i??" }
                    AccordionContent {
                        div { padding_bottom: "1rem",
                            p { padding: "0",
                                "A [insert age here] guy who likes Rust"                            }
                        }
                    }
                }
            }
        }

        }
    }

    }
}
