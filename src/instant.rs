use std::i64;

use crate::constants::*;
use crate::seconds_nanos::*;

#[cfg(test)]
pub mod factories;

/// An instantaneous point in time along the timeline.
///
/// This is explicitly a TAI instant.
/// This means that, among other things, the civil time an instant maps to
/// (a normal calendar date and time) changes over time, as leap seconds are added to the civil clock.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instant {
    epoch_second: i64,
    nanosecond_of_second: u32,
}

impl Instant {
    /// Constant for the earliest possible instant.
    pub const MIN: Instant = Instant {
        epoch_second: i64::MIN,
        nanosecond_of_second: 0,
    };

    /// Constant for the epoch instant, that is, '1970-01-01 00:00:00.000000000Z'.
    pub const EPOCH: Instant = Instant {
        epoch_second: 0,
        nanosecond_of_second: 0,
    };

    /// Constant for the last possible instant.
    pub const MAX: Instant = Instant {
        epoch_second: i64::MAX,
        nanosecond_of_second: NANOSECONDS_IN_SECOND as u32 - 1,
    };

    /// Obtains an Instant using milliseconds since '1970-01-01 00:00:00.000Z'.
    ///
    /// The seconds and fractional seconds are calculated from the provided milliseconds.
    ///
    /// # Parameters
    ///  - `epoch_milliseconds`: the milliseconds since the epoch.
    pub fn of_epoch_milli(epoch_milliseconds: i64) -> Instant {
        let (seconds, remainder) = (
            epoch_milliseconds / MILLISECONDS_IN_SECOND,
            epoch_milliseconds % MILLISECONDS_IN_SECOND,
        );
        let nanoseconds = remainder * NANOSECONDS_IN_MILLISECOND;
        Instant::of_epoch_second_and_adjustment_checked(seconds, nanoseconds)
            .expect("milliseconds would overflow instant")
    }

    /// Obtains an Instant using seconds since '1970-01-01 00:00:00Z'.
    ///
    /// # Parameters
    ///  - `epoch_seconds`: the seconds in the duration.
    pub fn of_epoch_second(epoch_seconds: i64) -> Instant {
        Instant::of_epoch_second_and_adjustment(epoch_seconds, 0)
    }

    /// Obtains an Instant using seconds and an adjustment in nanoseconds since '1970-01-01 00:00:00.000000000Z'.
    ///
    /// # Parameters
    ///  - `epoch_seconds`: the seconds since the epoch.
    ///  - `nano_adjustment`: the adjustment amount from the given second.
    ///
    /// # Panics
    /// - if the adjusted amount of seconds would be before the minimum instant, or after the maximum instant.
    pub fn of_epoch_second_and_adjustment(epoch_seconds: i64, nano_adjustment: i64) -> Instant {
        Instant::of_epoch_second_and_adjustment_checked(epoch_seconds, nano_adjustment)
            .expect("seconds would overflow instant")
    }

    fn of_epoch_second_and_adjustment_checked(
        seconds: i64,
        nano_adjustment: i64,
    ) -> Option<Instant> {
        of_seconds_and_adjustment_checked(seconds, nano_adjustment).map(|(seconds, nanos)| {
            Instant {
                epoch_second: seconds,
                nanosecond_of_second: nanos,
            }
        })
    }

    /// Gets the number of seconds before or after the epoch.
    ///
    /// [`nanos()`]: struct.Instant.html#method.nanos
    pub fn epoch_second(&self) -> i64 {
        self.epoch_second
    }

    /// Gets the number of nanoseconds farther along the timeline in this instant.
    ///
    /// [`epoch_seconds()`]: struct.Instant.html#method.epoch_seconds
    pub fn nano(&self) -> u32 {
        self.nanosecond_of_second
    }
}
