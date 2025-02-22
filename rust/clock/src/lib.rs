use core::fmt;
use std::ops::Rem;

#[derive(Debug, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes+minutes)
        }
    

    // accessor
    pub fn get_hours(&self) -> &i32 {
        &self.hours
    }

    pub fn get_minutes(&self) -> &i32 {
        &self.minutes
    }

    pub fn set_hour(&mut self, new_hours: i32) {
        self.hours = new_hours;
    }

    pub fn set_minutes(&mut self, new_minutes: i32) {
        self.minutes = new_minutes;
    }

    pub fn increment_hours(&mut self) {
        self.hours += 1;
    }
}



impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}
