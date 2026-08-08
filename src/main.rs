use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
use dioxus::prelude::*;

mod components;
use components::accordion::{
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
};

static ICON: Asset = asset!("/assets/pfp.jpeg");

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
            class: "min-h-screen bg-black text-white",

            div {
                class: "flex items-center gap-4",

                img {
                    class: "size-108 shadow-xl rounded-full",
                    src: ICON,
                }

                div {
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
                            },
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
        }
    }
}
}

fn calculate_age(birth: NaiveDateTime) -> String {
    let now = Local::now().naive_local();

    // Start with the rough number of years.
    let mut years = now.year() - birth.year();

    // Find this year's birthday.
    let mut birthday_this_year = NaiveDate::from_ymd_opt(
        now.year(),
        birth.month(),
        birth.day(),
    )
    .unwrap();

    // If this year's birthday hasn't happened yet,
    // subtract one year.
    if birthday_this_year > now.date() {
        years -= 1;

        birthday_this_year = NaiveDate::from_ymd_opt(
            now.year() - 1,
            birth.month(),
            birth.day(),
        )
        .unwrap();
    }

    // Now calculate months since the last birthday.
    let mut months =
        now.month() as i32 - birthday_this_year.month() as i32;

    if months < 0 {
        months += 12;
    }

    // Find the date after adding those months.
    let mut month_year = birthday_this_year.year();
    let mut month = birthday_this_year.month() as i32 + months;

    if month > 12 {
        month -= 12;
        month_year += 1;
    }

    let mut month_date = NaiveDate::from_ymd_opt(
        month_year,
        month as u32,
        birthday_this_year.day(),
    )
    .unwrap();

    // If that month anniversary hasn't happened yet,
    // go back one month.
    if month_date > now.date() {
        months -= 1;

        if months < 0 {
            months = 11;
        }

        if month == 1 {
            month = 12;
            month_year -= 1;
        } else {
            month -= 1;
        }

        month_date = NaiveDate::from_ymd_opt(
            month_year,
            month as u32,
            birthday_this_year.day(),
        )
        .unwrap();
    }

    // Calculate the remaining days.
    let days = (now.date() - month_date).num_days();

    // Midnight of the current day.
    let today_midnight = now
        .date()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    // Time elapsed since midnight.
    let seconds_today = (now - today_midnight).num_seconds();

    let hours = seconds_today / 3600;
    let minutes = (seconds_today % 3600) / 60;
    let seconds = seconds_today % 60;

    format!(
        "{} years, {} months, {} days, {} hours, {} minutes, {} seconds old",
        years,
        months,
        days,
        hours,
        minutes,
        seconds
    )
}