use std::fmt;

const DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32, 
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let remainder = (hours * 60 + minutes).rem_euclid(DAY);
        
        let hours = remainder / 60;
        let minutes = remainder - (hours * 60);
        
        Clock{ hours , minutes }

    }


    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
    
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

