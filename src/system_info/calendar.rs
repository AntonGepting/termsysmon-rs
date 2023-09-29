use chrono::prelude::*;
use chrono::{Datelike, NaiveDate, Weekday};

// hearts = red
// diamonds = blue
// clubs = green
// spikes = black
const CARDS: [&str; 54] = [
    "2♥", "3♥", "4♥", "5♥", "6♥", "7♥", "8♥", "9♥", "T♥", "J♥", "Q♥", "K♥", "A♥", "2♦", "3♦", "4♦",
    "5♦", "6♦", "7♦", "8♦", "9♦", "T♦", "J♦", "Q♦", "K♦", "A♦", "2♣", "3♣", "4♣", "5♣", "6♣", "7♣",
    "8♣", "9♣", "T♣", "J♣", "Q♣", "K♣", "A♣", "2♠", "3♠", "4♠", "5♠", "6♠", "7♠", "8♠", "9♠", "T♠",
    "J♠", "Q♠", "K♠", "A♠", "★", "NA",
];

// const CARDS: [&str; 54] = [
//     "2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "Th", "Jh", "Qh", "Kh", "Ah", "2d", "3d", "4d",
//     "5d", "6d", "7d", "8d", "9d", "Td", "Jd", "Qd", "Kd", "Ad", "2♣", "3♣", "4♣", "5♣", "6♣", "7♣",
//     "8c", "9c", "Tc", "Jc", "Qc", "Kc", "Ac", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "Ts",
//     "Js", "Qs", "Ks", "As", "Jo", "NA",
// ];
const CARDS_DEFAULT: usize = 53;

// i - week beginning with 1 [1..53]
pub fn get_card_by_num(i: usize) -> &'static str {
    CARDS.get(i - 1).unwrap_or(&CARDS[CARDS_DEFAULT])
}

// print date, highlight if current date
pub fn highlight_current_date(d: NaiveDate, today: NaiveDate) {
    if d == today {
        print!("\x1b[93;1;1m{:>2}\x1b[0m ", d.day())
    } else {
        print!("{:>2} ", d.day())
    }
}

pub fn print_cal_line(
    d0: &mut NaiveDate,
    d0_days: &mut usize,
    today: NaiveDate,
    new_line: bool,
    first_line: bool,
) {
    if new_line {
        if *d0_days > 0 {
            print!(
                "\x1b[38;5;243m{:>2} {}\x1b[0m ",
                d0.iso_week().week(),
                get_card_by_num(d0.iso_week().week() as usize)
            )
        } else {
            print!("       ");
        }
    }

    // pre- skip n cells (fill with spaces)
    if first_line {
        let skip = "   ".repeat(d0.weekday().num_days_from_monday() as usize);
        print!("{}", skip);
    }

    // if sunday end the line
    loop {
        if *d0_days > 0 {
            highlight_current_date(*d0, today)
        } else {
            print!("   ");
        }
        if d0.weekday() == Weekday::Sun {
            break;
        }
        *d0 = d0.succ_opt().unwrap();
        *d0_days = *d0_days - 1;
    }
}

pub fn print_calendar() {
    let now = Local::now();

    // Current local date
    let today = now.date_naive();

    //let today = NaiveDate::now();
    //let c = Utc::now();
    let mut d0 = NaiveDate::from_ymd_opt(2023, 8, 1).unwrap();
    let mut d1 = NaiveDate::from_ymd_opt(2023, 9, 1).unwrap();
    let mut d2 = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
    let mut d3 = NaiveDate::from_ymd_opt(2023, 11, 1).unwrap();

    //let days = d1.signed_duration_since(d0).num_days();

    println!(
        "      {}                      {}                   {}",
        d0.format("%B"),
        d1.format("%B"),
        d2.format("%B")
    );

    // print table header
    let s = "\x1b[38;5;243mCW  C Mo Tu We Th Fr Sa Su  \x1b[0m".repeat(3);
    print!("{}\n", s);

    let mut new_line = true;
    let mut first_line = true;

    let mut d0_days = (d1 - d0).num_days();
    let mut d1_days = (d2 - d1).num_days();
    let mut d2_days = (d3 - d2).num_days();

    for _ in 1..7 {
        if new_line {
            if d0_days > 0 {
                print!(
                    "\x1b[38;5;243m{:>2} {}\x1b[0m ",
                    d0.iso_week().week(),
                    get_card_by_num(d0.iso_week().week() as usize)
                )
            } else {
                print!("       ");
            }
        }

        // pre- skip n cells (fill with spaces)
        if first_line {
            let skip = "   ".repeat(d0.weekday().num_days_from_monday() as usize);
            print!("{}", skip);
        }

        // if sunday end the line
        loop {
            if d0_days > 0 {
                highlight_current_date(d0, today)
            } else {
                print!("   ");
            }
            if d0.weekday() == Weekday::Sun {
                break;
            }
            d0 = d0.succ_opt().unwrap();
            d0_days = d0_days - 1;
        }

        if new_line {
            if d1_days > 0 {
                print!(
                    "\x1b[38;5;243m{:>3} {}\x1b[0m ",
                    d1.iso_week().week(),
                    get_card_by_num(d1.iso_week().week() as usize)
                )
            } else {
                print!("      ");
            }
        }

        // pre- skip n cells (fill with spaces)
        if first_line {
            let skip = "   ".repeat(d1.weekday().num_days_from_monday() as usize);
            print!("{}", skip);
        }

        // print!("\x1b[{};{}H)", 10, 10);
        // if sunday end the line
        loop {
            if d1_days > 0 {
                highlight_current_date(d1, today)
            } else {
                print!("   ");
            }
            if d1.weekday() == Weekday::Sun {
                break;
            }
            d1 = d1.succ_opt().unwrap();
            d1_days = d1_days - 1;
        }

        if new_line {
            if d2_days > 0 {
                print!(
                    "\x1b[38;5;243m{:>3} {}\x1b[0m ",
                    d2.iso_week().week(),
                    get_card_by_num(d2.iso_week().week() as usize)
                )
            } else {
                print!("       ");
            }
        }

        // pre- skip n cells (fill with spaces)
        if first_line {
            let skip = "   ".repeat(d2.weekday().num_days_from_monday() as usize);
            print!("{}", skip);
        }

        // if sunday end the line
        loop {
            if d2_days > 0 {
                highlight_current_date(d2, today)
            } else {
                print!("   ");
            }
            if d2.weekday() == Weekday::Sun {
                break;
            }
            d2 = d2.succ_opt().unwrap();
            d2_days = d2_days - 1;
        }

        // if sunday end the line
        if d2.weekday() == Weekday::Sun {
            print!("\n");

            d0 = d0.succ_opt().unwrap();
            d0_days = d0_days - 1;

            d1 = d1.succ_opt().unwrap();
            d1_days = d1_days - 1;

            d2 = d2.succ_opt().unwrap();
            d2_days = d2_days - 1;

            new_line = true;
            first_line = false;
        }
    }
    println!();
}

pub fn calendar() {
    print_calendar();
}
