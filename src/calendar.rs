use std::time::SystemTime;
use std::fmt;
use std::option::Option;
use chrono::{DateTime, NaiveDateTime, NaiveDate, Local, Duration};


// TODO: LOAD + SAVE
// TODO: Export / import to other popular calendars?? integrate google api
// TODO: Docs


pub struct User {
    pub username: String,
    pub password: String,
    calendar: Calendar,
    next_event_id: u64,
}


pub fn create_user(username: &str, password: &str) -> User {
    User {
        username: String::from(username),
        password: String::from(password),
        calendar: Calendar{
            events: Vec::<Event>::new(),
        },
        next_event_id: 0,
    }
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


    pub fn create_allday_event(
        &mut self, 
        title: &str,
        desc: &str, 
        date: DateTime<Local>,
    ) {
        let mut evt = Event{
            id: self.generate_event_id(),
            time_start: date,
            time_end: Some(date + Duration::days(1)),
            title: String::from(title),
            kind: EventType::AllDay,
            desc: String::from(desc),
            reminders: Vec::new()
        };
        self.calendar.events.push(evt);
    }


    pub fn create_reminder_event(
        &mut self, 
        title: &str,
        desc: &str, 
        date_time: DateTime<Local>,
    ) {
        let mut evt = Event{
            id: self.generate_event_id(),
            time_start: date_time,
            time_end: Some(date_time),
            title: String::from(title),
            kind: EventType::Reminder,
            desc: String::from(desc),
            reminders: Vec::new()
        };
        self.calendar.events.push(evt);
    }


    pub fn create_timed_event(
        &mut self, 
        title: &str, 
        desc: &str, 
        time_start: DateTime<Local>,
        time_end: DateTime<Local>,
    ) {
        let mut evt = Event{
            id: self.generate_event_id(),
            time_start,
            time_end: Some(time_end),
            title: String::from(title),
            kind: EventType::Timed,
            desc: String::from(desc),
            reminders: Vec::new()
        };
        self.calendar.events.push(evt);
    }


    // edit event


    pub fn get_events(&self) -> &Vec<Event> {
        &self.calendar.events
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


impl fmt::Display for EventType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EventType::AllDay => write!(f, "All Day"),
            EventType::Reminder => write!(f, "Reminder"),
            EventType::Timed => write!(f, "Timed event"),
        }
    }
}


pub enum Repeat {   // monthly = 30 days or every month? What to do if last day of month is 31, next month max=29 monthly reminder -> 29?
    None,
    Daily,
    CustomWeekDays([bool; 7]),          // [true, false, .., false, true] = repeat every sunday and monday
    Workdays,
    Weekends,
    Weekly,
    CustomMonthWeeks([bool; 4]),        // [true, false, true, false] = repeat every 1st and 3rd week of a month
    Monthly,
    CustomYearMonths([bool; 12]),       // [true, false, .., false, true] = repeat every 1st and 12th month of a year
    Annually,
}


impl fmt::Display for Repeat{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Repeat::None => write!(f, "None"),
            Repeat::Daily => write!(f, "Daily"),
            Repeat::CustomWeekDays(..) => write!(f, "Custom weekdays"),
            Repeat::Workdays => write!(f, "Workdays"),
            Repeat::Weekends => write!(f, "Weekends"),
            Repeat::Weekly => write!(f, "Weekly"),
            Repeat::CustomMonthWeeks(..) => write!(f, "Custom weeks"),
            Repeat::Monthly => write!(f, "Monthly"),
            Repeat::CustomYearMonths(..) => write!(f, "Custom months"),
            Repeat::Annually => write!(f, "Annually"),
        }
    }
}