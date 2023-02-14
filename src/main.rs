#![allow(unused)]

//use std::io;
mod calendar;
mod cli;

use std::{time::SystemTime, alloc::System};
use std::fmt;
use std::option::Option;
use chrono::{DateTime, NaiveDateTime, NaiveDate, Local, Duration};
use calendar::*;
use cli::*;


fn test_fill() -> User {
    let mut start: DateTime::<Local> = SystemTime::now().into();
    start += Duration::hours(7);
    let end = start + Duration::hours(5);

    let mut usr = create_user("jija", "jija");
    usr.create_timed_event("Work", "jija kakaja-to", start, end);

    usr.add_reminder(1, Duration::hours(2));
    usr.add_reminder(1, Duration::minutes(30));

    usr
}


// TODO : shortcut for current time without casting?


fn main() {
    let mut usr = test_fill();

    usr.print_events();

    usr.print_reminders(1);

    usr.remove_event(1);

    usr.create_allday_event("jeeja", "jijuu", SystemTime::now().into());
    usr.create_reminder_event("pojijevat", "jijuu2", SystemTime::now().into());

    usr.print_events();

    usr.edit_event_title(2, "izmenenni");
    usr.edit_event_desc(2, "ivent");
    usr.edit_event_kind(2, EventType::Timed);

    usr.print_events();

    usr.edit_event_time_end(2, SystemTime::now().into());
}