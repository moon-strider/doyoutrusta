use std::time::SystemTime;
use std::fmt;
use std::option::Option;
use chrono::{DateTime, NaiveDateTime, NaiveDate, Local, Duration};


pub struct User {
    pub username: String,
    pub password: String,
    pub calendar: Calendar,
    pub next_event_id: u64,
}


impl User {
    pub fn remove_event(&mut self, id: u64) {
        if self.calendar.events.clone()
            .into_iter()
            .filter(|item| item.id == id)
            .collect::<Vec<Event>>()
            .len() == 0 {
            panic!("Event with specified id does not exist!");
        }

        self.calendar.events = self.calendar.events.clone()
            .into_iter()
            .filter(|item| item.id != id)
            .collect();
    }


    pub fn create_event(
        &mut self, 
        title: String, 
        desc: String, 
        kind: EventType,
        time_start: DateTime<Local>,
        time_end: Option<DateTime<Local>>,
        reminders: Vec<DateTime<Local>>
    ) {
        let mut evt = Event{
            id: self.generate_event_id(),
            time_start,
            time_end,
            title,
            kind,
            desc,
            reminders
        };
        self.calendar.events.push(evt);
    }


    fn generate_event_id(&mut self) -> u64 {
        self.next_event_id += 1;
        
        self.next_event_id
    }
}


pub struct Calendar {
    pub events: Vec<Event>,
}


#[derive(Clone)]
pub struct Event {
    pub id: u64,
    pub time_start: DateTime::<Local>,
    pub time_end: Option<DateTime::<Local>>,
    pub title: String,
    pub kind: EventType,
    pub desc: String,
    pub reminders: Vec<DateTime<Local>>,
}


#[derive(Clone)]
pub enum EventType {
    AllDay,
    Reminder,
    Timed
}


pub enum Repeat {
    r#None,
    Daily,
    CustomDaily,
    CustomWeekDays,
    Weekly,
    CustomWeekly,
    CustomMonthWeeks,
    Monthly,
    CustomMonthly,
    CustomYearMonths,
    Annually,
    CustomAnnually,
}


impl fmt::Display for EventType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EventType::AllDay => write!(f, "All Day"),
            EventType::Reminder => write!(f, "Reminder"),
            EventType::Timed => write!(f, "Timed event"),
        }
    }
}