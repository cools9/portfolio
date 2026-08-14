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

    // Work out completed years.
    let mut years = now.year() - birth.year();

    if (now.month(), now.day()) < (birth.month(), birth.day()) {
        years -= 1;
    }

    // Date of the most recent birthday.
    let birthday_year = birth.year() + years;

    let last_birthday = NaiveDate::from_ymd_opt(
        birthday_year,
        birth.month(),
        birth.day(),
    )
    .unwrap();

    // Work out completed months after the last birthday.
    let mut months =
        (now.year() - last_birthday.year()) * 12
        + now.month() as i32
        - last_birthday.month() as i32;

    if now.day() < last_birthday.day() {
        months -= 1;
    }

    // If we're still before the birthday/month anniversary,
    // make sure the value isn't negative.
    if months < 0 {
        months = 0;
    }

    // Find the date after those completed months.
    let total_months =
        last_birthday.year() * 12
        + last_birthday.month() as i32 - 1
        + months;

    let anniversary_year = total_months / 12;
    let anniversary_month = (total_months % 12) + 1;

    // Find how many days are in the anniversary month.
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

    let first_of_next_month = NaiveDate::from_ymd_opt(
        next_month_year,
        next_month as u32,
        1,
    )
    .unwrap();

    let first_of_month = NaiveDate::from_ymd_opt(
        anniversary_year,
        anniversary_month as u32,
        1,
    )
    .unwrap();

    let days_in_month =
        (first_of_next_month - first_of_month).num_days();

    // Keep the birthday day where possible.
    // For example, a 29th birthday in February becomes Feb 28
    // in a non-leap year.
    let anniversary_day =
        birth.day().min(days_in_month as u32);

    let anniversary_date = NaiveDate::from_ymd_opt(
        anniversary_year,
        anniversary_month as u32,
        anniversary_day,
    )
    .unwrap();

    // Put the original birth time onto the anniversary date.
    let anniversary = anniversary_date
        .and_hms_opt(
            birth.hour(),
            birth.minute(),
            birth.second(),
        )
        .unwrap();

    // Exact remaining time after years + months.
    let remaining = now.signed_duration_since(anniversary);

    let days = remaining.num_days();
    let hours = remaining.num_hours() % 24;
    let minutes = remaining.num_minutes() % 60;
    let seconds = remaining.num_seconds() % 60;

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