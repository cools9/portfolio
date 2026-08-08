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
            Accordion {
                AccordionItem {
                    index:0,
                    AccordionTrigger {
                        "About Me"
                    }

                    AccordionContent {
                        p {
                            "I'm a developer who likes building cool stuff."
                        }
                    }
                }

                AccordionItem {
                    index:1,
                    AccordionTrigger {
                        "Projects"
                    }

                    AccordionContent {
                        p {
                            class:"text-gray-900 dark:text-white mt-5 text-base font-medium tracking-tight",
                            "Here are some of my projects."
                        }
                    }
                }
            }
            }

        }
    }

    }
}
