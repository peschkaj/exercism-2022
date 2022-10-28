use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MINUTES_PER_DAY: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        let mut minutes = minutes;

        if minutes < 0 {
            hours += (minutes / 60) - 1;
            minutes = 60 - (i32::abs(minutes) % 60);
        }

        if hours < 0 {
            hours = 24 - (i32::abs(hours) % 24);
        }

        Clock { minutes: ((hours * 60) + minutes) % MINUTES_PER_DAY }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}