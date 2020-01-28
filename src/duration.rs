use std;
use std::fmt;
use std::i64;
use std::u32;

use std::ops::Neg;

use crate::constants::*;
use crate::seconds_nanos::*;

#[cfg(test)]
pub mod abs;
#[cfg(test)]
pub mod constants;
#[cfg(test)]
pub mod display;
#[cfg(test)]
pub mod factories;
#[cfg(test)]
pub mod neg;
#[cfg(test)]
pub mod test_util;
#[cfg(test)]
pub mod to;

/// A time-based amount of time, such as '34.5 seconds'.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration {
    seconds: i64,
    nanoseconds_of_second: u32,
}

impl Duration {
    /// Constant for a duration with the greatest negative length.
    pub const MIN: Duration = Duration {
        seconds: i64::MIN,
        nanoseconds_of_second: 0,
    };
    /// Constant for a duration of zero length.
    pub const ZERO: Duration = Duration {
        seconds: 0,
        nanoseconds_of_second: 0,
    };
    /// Constant for a duration with the greatest positive length.
    pub const MAX: Duration = Duration {
        seconds: i64::MAX,
        nanoseconds_of_second: NANOSECONDS_IN_SECOND as u32 - 1,
    };

    /// Obtains a `Duration` representing a number of standard days.
    ///
    /// The seconds are calculated based on the standard definition of a day, where each day is 86,400 seconds.
    /// The nanosecond in second field is set to zero.
    ///
    /// # Parameters
    ///  - `days`: the days in the duration.
    ///
    /// # Panics
    /// - if the amount of days would overflow the duration.
    pub fn of_days(days: i64) -> Duration {
        Duration::of_days_checked(days).expect("days would overflow duration")
    }

    fn of_days_checked(days: i64) -> Option<Duration> {
        days.checked_mul(SECONDS_IN_DAY)
            .map(|total_seconds| Duration::of_seconds(total_seconds))
    }

    /// Obtains a `Duration` representing a number of standard hours.
    ///
    /// The seconds are calculated based on the standard definition of an hour, where each hour is 3600 seconds.
    /// The nanosecond in second field is set to zero.
    ///
    /// # Parameters
    ///  - `hours`: the hours in the duration.
    ///
    /// # Panics
    /// - if the amount of hours would overflow the duration.
    pub fn of_hours(hours: i64) -> Duration {
        Duration::of_hours_checked(hours).expect("hours would overflow duration")
    }

    fn of_hours_checked(hours: i64) -> Option<Duration> {
        hours
            .checked_mul(SECONDS_IN_HOUR)
            .map(|total_seconds| Duration::of_seconds(total_seconds))
    }

    /// Obtains a `Duration` representing a number of standard minutes.
    ///
    /// The seconds are calculated based on the standard definition of a minute, where each minute is 60 seconds.
    /// The nanosecond in second field is set to zero.
    ///
    /// # Parameters
    ///  - `minutes`: the minutes in the duration.
    ///
    /// # Panics
    /// - if the amount of minutes would overflow the duration.
    pub fn of_minutes(minutes: i64) -> Duration {
        Duration::of_minutes_checked(minutes).expect("minutes would overflow duration")
    }

    fn of_minutes_checked(minutes: i64) -> Option<Duration> {
        minutes
            .checked_mul(SECONDS_IN_MINUTE)
            .map(|total_seconds| Duration::of_seconds(total_seconds))
    }

    /// Obtains a Duration representing a number of seconds and an adjustment in nanoseconds.
    ///
    /// # Parameters
    ///  - `seconds`: the seconds in the duration.
    ///  - `nano_adjustment`: the adjustment amount from the given second.
    ///
    /// # Panics
    /// - if the adjusted amount of seconds would overflow the duration.
    pub fn of_seconds_and_adjustment(seconds: i64, nano_adjustment: i64) -> Duration {
        Duration::of_seconds_and_adjustment_checked(seconds, nano_adjustment)
            .expect("nano adjustment would overflow duration")
    }

    fn of_seconds_and_adjustment_checked(seconds: i64, nano_adjustment: i64) -> Option<Duration> {
        of_seconds_and_adjustment_checked(seconds, nano_adjustment).map(|(seconds, nanos)| {
            Duration {
                seconds: seconds,
                nanoseconds_of_second: nanos,
            }
        })
    }

    /// Obtains a Duration representing a number of seconds.
    ///
    /// The nanosecond field will be set to 0.
    ///
    /// # Parameters
    ///  - `seconds`: the seconds in the duration.
    pub const fn of_seconds(seconds: i64) -> Duration {
        Duration {
            seconds: seconds,
            nanoseconds_of_second: 0,
        }
    }

    /// Obtains a `Duration` representing a number of milliseconds.
    ///
    /// The seconds and nanoseconds are extracted from the specified milliseconds.
    ///
    /// # Parameters
    ///  - `millis`: the milliseconds in the duration.
    pub fn of_millis(millis: i64) -> Duration {
        let seconds = millis / MILLISECONDS_IN_SECOND;
        let adjustment = (millis % MILLISECONDS_IN_SECOND) * NANOSECONDS_IN_MILLISECOND;

        let (second_adjustment, nanos) = carry_and_nanos(adjustment);

        Duration {
            seconds: seconds + second_adjustment,
            nanoseconds_of_second: nanos,
        }
    }

    /// Obtains a `Duration` representing a number of nanoseconds.
    ///
    /// The seconds and nanoseconds are extracted from the specified nanoseconds.
    ///
    /// # Parameters
    ///  - `nanos`: the nanos in the duration.
    pub fn of_nanos(nanoseconds: i64) -> Duration {
        let (seconds, nanos) = seconds_and_nanos(nanoseconds);
        Duration {
            seconds: seconds,
            nanoseconds_of_second: nanos,
        }
    }

    /// Gets the number of nanoseconds within the second in this duration.
    ///
    /// [`seconds()`]: struct.Duration.html#method.seconds
    pub const fn nano(&self) -> u32 {
        self.nanoseconds_of_second
    }

    /// Gets the number of seconds in this duration.
    ///
    /// [`nano()`]: struct.Duration.html#method.nano
    pub const fn seconds(&self) -> i64 {
        self.seconds
    }

    /// Returns a new duration with a positive length and matching amplitude.
    ///
    /// This method returns a positive duration by effectively removing the sign from any negative total length.
    /// For example, `PT-1.3S` will be returned as `PT1.3S`.
    ///
    /// # Panics
    /// - if the duration would overflow after removing the negative sign.
    pub fn abs(self) -> Duration {
        if self >= Duration::ZERO {
            self
        } else {
            checked_neg(self).expect("absolute value would overflow duration")
        }
    }

    /// The total number of days in the duration.
    ///
    /// This returns the total number of days in the duration by dividing the number of seconds by 86,400.
    /// This is based on the standard definition of a day as 24 hours.
    pub fn to_days(&self) -> i64 {
        self.seconds() / SECONDS_IN_DAY
    }

    /// The total number of hours in the duration.
    ///
    /// This returns the total number of hours in the duration by dividing the number of seconds by 3,600.
    /// This is based on the standard definition of an hour as 60 minutes.
    pub fn to_hours(&self) -> i64 {
        self.seconds() / SECONDS_IN_HOUR
    }

    /// The total number of minutes in the duration.
    ///
    /// This returns the total number of minutes in the duration by dividing the number of seconds by 60.
    /// This is based on the standard definition of a minute as 60 seconds.
    pub fn to_minutes(&self) -> i64 {
        self.seconds() / SECONDS_IN_MINUTE
    }

    /// The total number of milliseconds in the duration.
    ///
    /// This returns the total number of milliseconds in the duration by multiplying the number of seconds by 1,000.
    ///
    /// # Panics
    /// - if the amount of milliseconds would overflow an i64.
    pub fn to_millis(&self) -> i64 {
        self.seconds()
            .checked_mul(MILLISECONDS_IN_SECOND)
            .and_then(|result| result.checked_add(self.nano() as i64 / NANOSECONDS_IN_MILLISECOND))
            .expect("total milliseconds would overflow")
    }

    /// The total number of nanoseconds in the duration.
    ///
    /// This returns the total number of nanoseconds in the duration by multiplying the number of seconds by 1,000,000,000.
    ///
    /// # Panics
    /// - if the amount of nanoseconds would overflow an i64.
    pub fn to_nanos(&self) -> i64 {
        self.seconds()
            .checked_mul(NANOSECONDS_IN_SECOND)
            .and_then(|result| result.checked_add(self.nano() as i64))
            .expect("total nanoseconds would overflow")
    }
}

impl fmt::Display for Duration {
    /// A string representation of this duration using ISO-8601 seconds based representation,
    /// such as PT8H6M12.345S.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &Duration::ZERO {
            return f.write_str("PT0S");
        }

        f.write_str("PT")?;

        let (effective_seconds, directed_nanos) = if self.seconds() >= 0 || self.nano() == 0 {
            (self.seconds(), self.nano())
        } else {
            (
                self.seconds() + 1,
                NANOSECONDS_IN_SECOND as u32 - self.nano(),
            )
        };

        let hours = effective_seconds / SECONDS_IN_HOUR;
        if hours != 0 {
            hours.fmt(f)?;
            f.write_str("H")?;
        }

        let minutes = (effective_seconds % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE;
        if minutes != 0 {
            minutes.fmt(f)?;
            f.write_str("M")?;
        }

        let remaining_seconds = effective_seconds % SECONDS_IN_MINUTE;
        if remaining_seconds != 0 || directed_nanos != 0 {
            if remaining_seconds == 0 && effective_seconds < 0 {
                f.write_str("-0")?;
            } else {
                remaining_seconds.fmt(f)?;
            }

            if directed_nanos != 0 {
                let formatted = format!(".{:09}", directed_nanos);
                f.write_str(formatted.as_str().trim_end_matches('0'))?;
            }
            f.write_str("S")?;
        }

        Ok(())
    }
}

impl Neg for Duration {
    type Output = Duration;

    /// Returns a copy of this duration with the length negated.
    ///
    /// This method swaps the sign of the total length of this duration.
    /// For example, `PT1.3S` will be returned as `PT-1.3S`.
    ///
    /// # Panics
    ///  - if swapping the sign would cause the duration to overflow.
    fn neg(self) -> Duration {
        checked_neg(self).expect("negated value would overflow duration")
    }
}

fn checked_neg(duration: Duration) -> Option<Duration> {
    match (duration.seconds(), duration.nano()) {
        (i64::MIN, 0) => None,
        (i64::MIN, nanos) => Some(Duration {
            seconds: i64::max_value(),
            nanoseconds_of_second: NANOSECONDS_IN_SECOND as u32 - nanos,
        }),
        (seconds, nanos) => {
            let (adjustment, flipped_nanos) = carry_and_nanos(-(nanos as i64));
            Some(Duration {
                seconds: -seconds + adjustment,
                nanoseconds_of_second: flipped_nanos,
            })
        }
    }
}
