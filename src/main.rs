use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
static ICON: Asset = asset!("/assets/pfp.jpeg");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { 
         document::Stylesheet { href: CSS }
        div {  
            class: "bg-red-500 w-64 h-64",
            img { 
                class: "w-64 h-64 object-cover rounded-full",
                src:ICON
             }

        }
     }
}