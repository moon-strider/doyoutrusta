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
    let mut evt: Vec<Event> = Vec::new();
    let mut end: DateTime::<Local> = SystemTime::now().into();
    end += Duration::days(1);

    evt.push(Event{
        id: 0,
        title: "Work".to_string(),
        kind: calendar::EventType::Reminder,
        desc: "I have to do it everyday".to_string(),
        reminders: Vec::new(),
        time_start: SystemTime::now().into(),
        time_end: Some(end)
    });

    let cld = Calendar {
        events: evt,
    };

    let usr = User {
        username: "Ilia".to_string(),
        password: "jija".to_string(),
        calendar: cld,
        next_event_id: 0
    };

    return usr
}


fn main() {
    let mut usr = test_fill();

    usr.print_events();

    usr.remove_event(0);

    usr.create_event("jeeja".to_string(), "jijuu".to_string(), EventType::AllDay, SystemTime::now().into(), None, Vec::new());
    usr.create_event("jeeja2".to_string(), "jijuu2".to_string(), EventType::Reminder, SystemTime::now().into(), None, Vec::new());

    usr.print_events();
}