use chrono::{DateTime, NaiveDateTime, NaiveDate, Local, Duration};
use crate::calendar::*;

impl User {
    pub fn print_events(&self) {
        println!("Events for user {}", self.username);
        for item in self.get_events() {
            println!("{0}: {1}, type: {2}, time_start: {3}, time_end: {4}, id: {5}", 
                item.title, 
                item.desc, 
                item.kind, 
                match item.kind {
                    EventType::Timed | EventType::Reminder => item.time_start.format("%d/%m/%Y %T"),
                    EventType::AllDay => item.time_start.format("%d/%m/%Y"),
                },
                match item.kind {
                    EventType::Timed => item.time_end.unwrap_or(item.time_start).format("%d/%m/%Y %T").to_string(),
                    EventType::AllDay => item.time_end.unwrap_or(item.time_start + Duration::days(1)).format("%d/%m/%Y").to_string(),
                    EventType::Reminder => item.time_end.unwrap_or(item.time_start).format("%d/%m/%Y %T").to_string(),
                },
                item.id,
            );
        }
    }

    pub fn print_reminders(&self, id: u64) { // revisit -> allow use of reminders > 1 day?
        let (title, reminders) = self.get_reminders_by_id(id);
        println!("Reminders for event \"{}\":", title);

        let mut sorted = reminders.clone();
        sorted.sort_by_key(|x| x.num_minutes());

        for reminder in sorted {
            let printable = reminder.num_minutes().to_string();
            println!("{} minutes", printable);
        }
    }
}