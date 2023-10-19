use chrono::prelude::*;
use chrono::{Datelike, NaiveDate, Weekday};

// hearts = red
// diamonds = blue
// clubs = green
// spikes = black
// const CARDS: [&str; 54] = [
//     "2♥", "3♥", "4♥", "5♥", "6♥", "7♥", "8♥", "9♥", "T♥", "J♥", "Q♥", "K♥", "A♥", "2♦", "3♦", "4♦",
//     "5♦", "6♦", "7♦", "8♦", "9♦", "T♦", "J♦", "Q♦", "K♦", "A♦", "2♣", "3♣", "4♣", "5♣", "6♣", "7♣",
//     "8♣", "9♣", "T♣", "J♣", "Q♣", "K♣", "A♣", "2♠", "3♠", "4♠", "5♠", "6♠", "7♠", "8♠", "9♠", "T♠",
//     "J♠", "Q♠", "K♠", "A♠", "★", "NA",
// ];
const CARDS: [&str; 54] = [
    "2󰣐", "3󰣐", "4󰣐", "5󰣐", "6󰣐", "7󰣐", "8󰣐", "9󰣐", "T󰣐", "J󰣐", "Q󰣐", "K󰣐", "A󰣐", "2󰣏", "3󰣏", "4󰣏",
    "5󰣏", "6󰣏", "7󰣏", "8󰣏", "9󰣏", "T󰣏", "J󰣏", "Q󰣏", "K󰣏", "A󰣏", "2󰣎", "3󰣎", "4󰣎", "5󰣎", "6󰣎", "7󰣎",
    "8󰣎", "9󰣎", "T󰣎", "J󰣎", "Q󰣎", "K󰣎", "A󰣎", "2󰣑", "3󰣑", "4󰣑", "5󰣑", "6󰣑", "7󰣑", "8󰣑", "9󰣑", "T󰣑",
    "J󰣑", "Q󰣑", "K󰣑", "A󰣑", "󰓎", "NA",
];

// const CARDS: [&str; 54] = [
//     "2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "Th", "Jh", "Qh", "Kh", "Ah", "2d", "3d", "4d",
//     "5d", "6d", "7d", "8d", "9d", "Td", "Jd", "Qd", "Kd", "Ad", "2♣", "3♣", "4♣", "5♣", "6♣", "7♣",
//     "8c", "9c", "Tc", "Jc", "Qc", "Kc", "Ac", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "Ts",
//     "Js", "Qs", "Ks", "As", "Jo", "NA",
// ];
const CARDS_DEFAULT: usize = 53;
const GREY_COLOR: &str = "\x1b[38;5;243m";
//const HIGHLIGHT_COLOR: &str = "\x1b[93;1;1m";
const HIGHLIGHT_COLOR: &str = "\x1b[1;48;0;3m";
const DEFAULT_COLOR: &str = "\x1b[0m";

// i - week beginning with 1 [1..53]
pub fn get_card_by_num(i: usize) -> &'static str {
    CARDS.get(i - 1).unwrap_or(&CARDS[CARDS_DEFAULT])
}

// print date, highlight if current date
pub fn highlight_current_date(d: NaiveDate, today: NaiveDate) {
    if d == today {
        print!("{}{:>2}{} ", HIGHLIGHT_COLOR, d.day(), DEFAULT_COLOR)
    } else {
        print!("{:>2} ", d.day())
    }
}

// print single line for 3 month calender with selected in the middle
pub fn print_cal_line(
    date: &mut NaiveDate,
    days: &mut i64,
    today: NaiveDate,
    new_line: bool,
    first_line: bool,
) {
    // every new line begins with header
    if new_line {
        // until days in current month not end, print line header, else empty header
        if *days > 0 {
            // week number & card symbol
            print!(
                "{}{:>2} {}{} ",
                GREY_COLOR,
                date.iso_week().week(),
                get_card_by_num(date.iso_week().week() as usize),
                DEFAULT_COLOR
            )
        } else {
            print!("      ");
        }
    }

    // pre- skip n cells (fill with spaces)
    if first_line {
        let skip = "   ".repeat(date.weekday().num_days_from_monday() as usize);
        print!("{}", skip);
    }

    // if sunday end the line
    loop {
        // until days in current month not end, print date in current cell, else empty cell
        if *days > 0 {
            highlight_current_date(*date, today)
        } else {
            print!("   ");
        }
        // if end of current week, print padding & exit
        if date.weekday() == Weekday::Sun {
            print!(" ");
            break;
        }
        // next day, decrement days in month
        *date = date.succ_opt().unwrap();
        *days = *days - 1;
    }
}

/// print 3 month in the row, selected in the middle position
// FIXME: 11, 12, 1 months
pub fn print_calendar(year: i32, month: u32) {
    let now = Local::now();

    // Current local date
    let today = now.date_naive();

    // date 0..3, different month for later use
    let mut d0 = if month - 1 >= 1 {
        NaiveDate::from_ymd_opt(year, month - 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year - 1, 12, 1).unwrap()
    };
    let mut d1 = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let mut d2 = if month + 1 <= 12 {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    };
    let d3 = if month + 2 <= 12 {
        NaiveDate::from_ymd_opt(year, month + 2, 1).unwrap()
    } else if month + 1 <= 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year + 1, 2, 1).unwrap()
    };

    //let days = d1.signed_duration_since(d0).num_days();

    // print month as full text
    println!(
        "      {:<10}                  {:<10}                  {:<10}",
        d0.format("%B"),
        d1.format("%B"),
        d2.format("%B")
    );

    // print table header
    let s = format!(
        "{}CW  C Mo Tu We Th Fr Sa Su  {}",
        GREY_COLOR, DEFAULT_COLOR
    )
    .repeat(3);
    print!("{}\n", s);

    let mut new_line = true;
    let mut first_line = true;

    // calc days in each month
    let mut d0_days = (d1 - d0).num_days();
    let mut d1_days = (d2 - d1).num_days();
    let mut d2_days = (d3 - d2).num_days();

    // println!("{} {} {}", &d0_days, &d1_days, &d2_days);

    // for 6 lines of calendar
    for _ in 1..7 {
        // print month blocks, line by line
        print_cal_line(&mut d0, &mut d0_days, today, new_line, first_line);
        print_cal_line(&mut d1, &mut d1_days, today, new_line, first_line);
        print_cal_line(&mut d2, &mut d2_days, today, new_line, first_line);

        // if sunday end the line
        if d2.weekday() == Weekday::Sun {
            print!("\n");

            d0 = d0.succ_opt().unwrap();
            d0_days = d0_days - 1;

            d1 = d1.succ_opt().unwrap();
            d1_days = d1_days - 1;

            d2 = d2.succ_opt().unwrap();
            d2_days = d2_days - 1;

            // begin new line
            new_line = true;
            // first line passed
            first_line = false;
        }
    }
    println!();
}

fn calendar_year(year: i32) {
    // println!("{}", year);
    print_calendar(year, 2);
    print_calendar(year, 5);
    print_calendar(year, 8);
    print_calendar(year, 11);
}

#[test]
fn calendar() {
    // print_calendar(2024, 2);

    let now = Local::now();
    let today = now.date_naive();
    println!(
        "{} {})",
        today.format("%d. %B %Y (CW: %W"),
        get_card_by_num(today.iso_week().week() as usize)
    );

    calendar_year(2023);
}
