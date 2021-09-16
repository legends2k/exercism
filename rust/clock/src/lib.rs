use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    min: i32,
}

// Return divisor-signed remainder
fn kmod(value: i32, max: i32) -> i32 {
    ((value % max) + max) % max
}

// Return quotient and divisor-signed remainder
fn div_rem(dividend: i32, divisor: i32) -> (i32, i32) {
    let rem = kmod(dividend, divisor);
    // Integer division (dividend ÷ divisor) rounds to 0, while we want to round
    // to −∞ i.e. floor() but instead of operating in float-space, we can simply
    // subtract 1 from quotient in case of negative dividends with remainder > 0
    let floor = if (rem != 0) && (dividend < 0) { -1 } else { 0 };
    ((dividend / divisor) + floor, rem)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (extra_hours, mins) = div_rem(minutes, 60);
        let (_, hrs) = div_rem(hours + extra_hours, 24);
        Clock {
            hour: hrs,
            min: mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hour, self.min + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.min)
    }
}
