extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameTime {
    pub day: usize,
    pub min: usize,
}

impl GameTime {
    pub const DAY_INTERVAL: usize = 1440;
    pub const HR_INTERVAL: usize = 60;
    pub const DAY_HALF: usize = 12;

    pub fn advance(&mut self, n: usize) {
        let days = n / Self::DAY_INTERVAL;
        let mins = n % Self::DAY_INTERVAL;

        self.day += days;
        self.min += mins;

        if self.min > Self::DAY_INTERVAL {
            self.min -= Self::DAY_INTERVAL;
            self.day += 1;
        }
    }

    pub fn hrs(&self) -> usize {
        self.min / Self::HR_INTERVAL
    }

    pub fn is_night(&self) -> bool {
        self.min > 1200 || self.min < 360
    }

    pub fn display_time(&self) -> String {
        let hr = self.hrs();
        let pm = hr > Self::DAY_HALF;

        format!("{}:{:0>2} {}",
            if pm { 
                hr - Self::DAY_HALF 
            } else {
                if hr == 0 {
                    12
                } else {
                    hr
                }
            },
            self.min % Self::HR_INTERVAL,
            if pm { "PM" } else { "AM" }
        )
    }

    pub fn display(&self) -> String {
        format!("The time is {}. It is day {} of your journey.",
            self.display_time(),
            self.day
        )
    }

    pub fn duration(d: usize) -> String {
        if d < Self::HR_INTERVAL {
            format!("{} minutes", d)
        } else if d < Self::DAY_INTERVAL {
            format!("{} hours and {} minutes", 
                d / Self::HR_INTERVAL,
                d % Self::HR_INTERVAL
            )
        } else {
            format!("{} days, {} hours, and {} minutes",
                d / Self::DAY_INTERVAL,
                d / Self::HR_INTERVAL,
                d % Self::HR_INTERVAL
            )
        }
    }
}