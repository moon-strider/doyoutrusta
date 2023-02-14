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
    let mut end: DateTime::<Local> = SystemTime::now().into();
    end += Duration::hours(1);

    let mut usr = create_user("jija", "jija");
    usr.create_timed_event("Work", "jija kakaja-to", SystemTime::now().into(), end);

    return usr
}


fn main() {
    let mut usr = test_fill();

    usr.print_events();

    usr.remove_event(1);

    usr.create_allday_event("jeeja", "jijuu", SystemTime::now().into());
    usr.create_reminder_event("pojijevat", "jijuu2", SystemTime::now().into());

    usr.print_events();
}