use notify_rust::{Notification};
use std::fs;
use dirs::home_dir;
use chrono::{Local, NaiveDate, Datelike, offset::TimeZone};

fn main() {
    let path = home_dir().unwrap();
    let today = Local::today();
    let calendar = fs::read_to_string(path.into_os_string().into_string().unwrap() + "/calendar.txt")
        .expect("Something went wrong reading the file");
    let calendar_line = calendar.lines();
    for (_i, line) in calendar_line.enumerate() {
        let mut iter = line.split(":");
        let name = iter.next().unwrap();
        let date_str = iter.next().unwrap();

        let date_year = format!("{} {}", date_str, today.year());

        let date = NaiveDate::parse_from_str(&date_year, "%d %B %Y").unwrap();

        if Local.from_local_date(&date).unwrap() == today {
            Notification::new()
            .summary("Today's event")
            .body(&"Today is {}'s birthday! Tell them HBD.".replace("{}", name))
            .appname("Birthday Reminder")
            .timeout(0)
            .show();
        }

    }
    
}
