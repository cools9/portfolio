use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
use dioxus::prelude::*;

use dioxus_icons::lucide::GitBranch;

use crate::components::card::Card;
use crate::components::card::CardContent;
use crate::components::card::CardDescription;
use crate::components::card::CardFooter;
use crate::components::card::CardHeader;
use crate::components::card::CardTitle;

//use crate::components::button::{Button, ButtonVariant};
mod components;
use components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};

static ICON: Asset = asset!("/assets/pfp.jpeg");

#[derive(Clone, PartialEq)]
struct Project {
    title: &'static str,
    description: &'static str,
    url: &'static str,
}

fn projects() -> Vec<Project> {
    vec![
        Project {
            title: "This Site",
            description: "Yes this site which youre looking at! is made by me using Rust and Dioxus.",
            url: "https://github.com/cools9/portfolio"
        },
        Project {
            title: "ARRFPS",
            description: "A Relatively Realistic First Person Shooter(ARRFPS) game i made using godot engine",
            url: "https://github.com/cools9/ARRFPS-A-Relatively-Realistic-First-Person-Shooter-"
        },
        Project {
            title: "AlooShaders",
            description: "My first Mein Kampf Shaders made by me.",
            url: "https://github.com/cools9/AlooShaders"
        },
    ]
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut age = use_signal(|| String::from("Calculating..."));

    use_effect(move || {
        spawn(async move {
            loop {
                let birth = NaiveDate::from_ymd_opt(2011, 8, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();

                age.set(calculate_age(birth));

                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        });
    });

    rsx! {
            document::Stylesheet {
                href: asset!("/assets/tailwind.css")
            }
           
            div {
        class: "min-h-screen bg-gray-950 text-white flex flex-col justify-between font-maple",

        div {
            class: "flex items-start gap-4",

            img {
                class: "size-108 shadow-xl rounded-full",
                src: ICON,
            }

            div {
                class: "flex flex-col gap-4",

                h1 {
                    class: "text-4xl",
                    "Cools9"
                }

                Accordion {
                    allow_multiple_open: false,
                    horizontal: false,

                    AccordionItem {
                        index: 0,

                        AccordionTrigger {
                            "Who am I??"
                        }

                        AccordionContent {
                            div {
                                padding_bottom: "1rem",

                                p {
                                    padding: "0",
                                    "A guy who is "
                                    span {
                                        class: "text-gray-400",
                                        "{age}"
                                    }
                                    " and likes Rust."
                                }
                            }
                        }
                    }

                    AccordionItem {
                        index: 1,

                        AccordionTrigger {
                            "What am i??"
                        }

                        AccordionContent {
                            div {
                                padding_bottom: "1rem",

                                p {
                                    padding: "0",
                                    "A homo sapien who writes awful code in Rust,Python and Go"
                                }
                            }
                        }
                    }
                }
            }
        }

       div {
        class: "flex flex-col items-center pb-6",

        h1 {
            class: "relative top-0 w-fit h-auto py-4 justify-center flex bg-gradient-to-r items-center from-blue-500 via-teal-500 to-pink-500 bg-clip-text text-6xl font-extrabold text-transparent text-center select-auto",
            "Wanna look at my projects..........."
        }

        div {
            class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6 p-6 w-full",

            for project in projects() {
                Card {
                    key: "{project.title}",

                    CardHeader {
                        CardTitle { "{project.title}" }
                        CardDescription { "{project.description}" }
                    }
                    CardFooter {
                        
                        button {
                            class: "inline-flex items-center gap-2 rounded-lg bg-gray-900 px-4 h-10 text-sm font-medium text-white shadow-sm hover:bg-gray-800 transition-all",
                            onclick: move |_| {
                                web_sys::window()
                                    .unwrap()
                                    .location()
                                    .set_href({project.url});
                            },
                            GitBranch {
                                size: 18,
                                stroke: "#ffffff",
                                stroke_width: 2
                            }

                            "GitHub"

                            
                        }

                    }

            }
        }
    }
    }

        }
    }
}

fn calculate_age(birth: NaiveDateTime) -> String {
    let now = Local::now().naive_local();

    let mut years = now.year() - birth.year();

    if (now.month(), now.day()) < (birth.month(), birth.day()) {
        years -= 1;
    }

    let birthday_year = birth.year() + years;

    let last_birthday = NaiveDate::from_ymd_opt(birthday_year, birth.month(), birth.day()).unwrap();

    let mut months = (now.year() - last_birthday.year()) * 12 + now.month() as i32
        - last_birthday.month() as i32;

    if now.day() < last_birthday.day() {
        months -= 1;
    }

    if months < 0 {
        months = 0;
    }

    let total_months = last_birthday.year() * 12 + last_birthday.month() as i32 - 1 + months;

    let anniversary_year = total_months / 12;
    let anniversary_month = (total_months % 12) + 1;

    let next_month_year = if anniversary_month == 12 {
        anniversary_year + 1
    } else {
        anniversary_year
    };

    let next_month = if anniversary_month == 12 {
        1
    } else {
        anniversary_month + 1
    };

    let first_of_next_month =
        NaiveDate::from_ymd_opt(next_month_year, next_month as u32, 1).unwrap();

    let first_of_month =
        NaiveDate::from_ymd_opt(anniversary_year, anniversary_month as u32, 1).unwrap();

    let days_in_month = (first_of_next_month - first_of_month).num_days();

    let anniversary_day = birth.day().min(days_in_month as u32);

    let anniversary_date =
        NaiveDate::from_ymd_opt(anniversary_year, anniversary_month as u32, anniversary_day)
            .unwrap();

    let anniversary = anniversary_date
        .and_hms_opt(birth.hour(), birth.minute(), birth.second())
        .unwrap();

    let remaining = now.signed_duration_since(anniversary);

    let days = remaining.num_days();
    let hours = remaining.num_hours() % 24;
    let minutes = remaining.num_minutes() % 60;
    let seconds = remaining.num_seconds() % 60;

    format!(
        "{} years, {} months, {} days, {} hours, {} minutes, {} seconds old",
        years, months, days, hours, minutes, seconds
    )
}
