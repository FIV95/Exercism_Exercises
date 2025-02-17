use core::fmt;

#[derive(Debug,Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours: hours % 24, minutes: if minutes == 60 {0} else {minutes} }
    }
    

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let mut hours_to_add: i32 = 0;
        let mut minutes_left= minutes;
        while minutes_left >= 60 {
            minutes_left = minutes_left - 60;
            hours_to_add += 1
        }
        while hours + hours_to_add >= 24 {
            if hours + hours_to_add == 24 {
                hours = 0;
                return Clock{hours,minutes: minutes_left}
            }
            hours += 1;
            hours_to_add -= 1;
        }
        return Clock{hours, minutes: minutes_left};
    }

    // accessor
    pub fn get_hours(&self) -> &i32 {&self.hours}

    pub fn get_minutes(&self) -> &i32 {&self.minutes}

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
        self.minutes == other.minutes && self.hours == other.hours}
}
