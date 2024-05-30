//! Provides functionality to create holiday calendars and perform financial date manipulation.
//!

use chrono::prelude::*;
use indexmap::set::IndexSet;
use std::collections::{HashSet};
use chrono::{Days, Weekday};
use pyo3::pyclass;

/// Struct for defining a holiday calendar.
///
/// A holiday calendar is formed of 2 components:
///
/// - `week_mask`: which defines the days of the week that are not general business days. In Western culture these
///   are typically `[5, 6]` for Saturday and Sunday.
/// - `holidays`: which defines specific dates that may be exceptions to the general working week, and cannot be
///   business days.
///
#[pyclass]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct HolCal {
    holidays: IndexSet<NaiveDateTime>,
    week_mask: HashSet<Weekday>,
}

impl HolCal {

    /// Create a holiday calendar.
    ///
    /// `holidays` provide a vector of dates that cannot be business days. `week_mask` is a vector of days
    /// (0=Mon,.., 6=Sun) that are excluded from the working week.
    pub fn new(holidays: Vec<NaiveDateTime>, week_mask: Vec<u8>) -> Self {
        HolCal {
            holidays: IndexSet::from_iter(holidays),
            week_mask: HashSet::from_iter(week_mask.into_iter().map(|v| Weekday::try_from(v).unwrap())),
        }
    }
}


/// Struct for defining a holiday calendar which is the union of two or more other calendars,
/// with the additional constraint of also ensuring settlement compliance with one or more
/// other calendars.
///
/// When the union of a holiday calendar is observed the following are true:
///
/// - a weekday is such if it is a weekday in all calendars.
/// - a holiday is such if it is a holiday in any calendar.
/// - a business day is such if it is a business day in all calendars.
#[pyclass]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct UnionCal {
    calendars: Vec<HolCal>,
    settlement_calendars: Option<Vec<HolCal>>,
}

impl UnionCal {
    pub fn new(calendars: Vec<HolCal>, settlement_calendars: Option<Vec<HolCal>>) -> Self {
        UnionCal { calendars, settlement_calendars }
    }
}

/// A trait to control business day management and date rolling.
pub trait DateRoll {

    /// Returns whether the date is part of the general working week.
    fn is_weekday(&self, date: &NaiveDateTime) -> bool;

    /// Returns whether the date is a specific holiday excluded from the regular working week.
    fn is_holiday(&self, date: &NaiveDateTime) -> bool;

    /// Returns whether the date is valid relative to an associated settlement calendar.
    ///
    /// If the holiday calendar object has no associated settlement calendar this should return `true`
    /// for any date.
    fn is_settlement(&self, date: &NaiveDateTime) -> bool;

    /// Returns whether the date is a business day, i.e. part of the working week and not a holiday.
    fn is_bus_day(&self, date: &NaiveDateTime) -> bool {
        self.is_weekday(date) && !self.is_holiday(date)
    }

    /// Returns whether the date is not a business day, i.e. either not in working week or a specific holiday.
    fn is_non_bus_day(&self, date: &NaiveDateTime) -> bool {
        !self.is_bus_day(date)
    }

    /// Return the date, if a business day, or get the proceeding business date.
    fn next_bus_day(&self, date: &NaiveDateTime) -> NaiveDateTime {
        let mut new_date = date.clone();
        while !self.is_bus_day(&new_date) {
            new_date = new_date + Days::new(1);
        }
        new_date
    }

    /// Return the date, if a business day, or get the preceeding business date.
    fn prev_bus_day(&self, date: &NaiveDateTime) -> NaiveDateTime {
        let mut new_date = date.clone();
        while !self.is_bus_day(&new_date) {
            new_date = new_date - Days::new(1);
        }
        new_date
    }

    /// Return the date, if a business day, or get the proceeding business date, without rolling
    /// into a new month.
    fn mod_next_bus_day(&self, date: &NaiveDateTime) -> NaiveDateTime {
        let new_date = self.next_bus_day(date);
        if new_date.month() != date.month() { self.prev_bus_day(date) } else { new_date }
    }

    /// Return the date, if a business day, or get the proceeding business date, without rolling
    /// into a new month.
    fn mod_prev_bus_day(&self, date: &NaiveDateTime) -> NaiveDateTime {
        let new_date = self.prev_bus_day(date);
        if new_date.month() != date.month() { self.next_bus_day(date) } else { new_date }
    }

}

impl DateRoll for HolCal {

    fn is_weekday(&self, date: &NaiveDateTime) -> bool {
        !self.week_mask.contains(&date.weekday())
    }

    fn is_holiday(&self, date: &NaiveDateTime) -> bool {
        self.holidays.contains(date)
    }

    fn is_settlement(&self, _date: &NaiveDateTime) -> bool {
        true
    }

}

impl DateRoll for UnionCal {

    fn is_weekday(&self, date: &NaiveDateTime) -> bool {
        self.calendars.iter().all(|cal| cal.is_weekday(date))
    }

    fn is_holiday(&self, date: &NaiveDateTime) -> bool {
        self.calendars.iter().any(|cal| cal.is_holiday(date))
    }

    fn is_settlement(&self, date: &NaiveDateTime) -> bool {
        match &self.settlement_calendars {
            None => true,
            Some(cals) => !cals.iter().any(|cal| cal.is_holiday(date))
        }
    }

}

/// Enum defining the rule to adjust a non-business day to a business day.
pub enum Modifier {
    /// Actual: date is unchanged, even if it is a non-business day.
    Act,
    /// Following: date is rolled to the next business day.
    F,
    /// Modified following: date is rolled to the next except if it changes month.
    ModF,
    /// Previous: date is rolled to the previous business day.
    P,
    /// Modified previous: date is rolled to the previous except if it changes month.
    ModP,
}

pub fn adjust(date: &NaiveDateTime, cal: &dyn DateRoll, modifier: &Modifier) -> NaiveDateTime {
    match modifier {
        Modifier::Act => date.clone(),
        Modifier::F => cal.next_bus_day(date),
        Modifier::P => cal.prev_bus_day(date),
        Modifier::ModF => cal.mod_next_bus_day(date),
        Modifier::ModP => cal.mod_prev_bus_day(date),
    }
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_hol_cal() -> HolCal {
        let hols = vec![
            NaiveDateTime::parse_from_str("2015-09-05 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),  // saturday
            NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),  // monday
        ];
        HolCal::new(hols, vec![5, 6])
    }

    #[test]
    fn test_is_holiday() {
        let cal = fixture_hol_cal();
        let hol = NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let no_hol = NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let saturday = NaiveDateTime::parse_from_str("2024-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(cal.is_holiday(&hol));  // In hol list
        assert!(!cal.is_holiday(&no_hol));  // Not in hol list
        assert!(!cal.is_holiday(&saturday));  // Not in hol list
    }

    #[test]
    fn test_is_weekday() {
        let cal = fixture_hol_cal();
        let hol = NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let no_hol = NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let saturday = NaiveDateTime::parse_from_str("2024-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let sunday = NaiveDateTime::parse_from_str("2024-01-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(cal.is_weekday(&hol));  // Monday
        assert!(cal.is_weekday(&no_hol));  //Thursday
        assert!(!cal.is_weekday(&saturday));  // Saturday
        assert!(!cal.is_weekday(&sunday));  // Sunday
    }

    #[test]
    fn test_is_business_day() {
        let cal = fixture_hol_cal();
        let hol = NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let no_hol = NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let saturday = NaiveDateTime::parse_from_str("2024-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(!cal.is_bus_day(&hol));  // Monday in Hol list
        assert!(cal.is_bus_day(&no_hol));  //Thursday
        assert!(!cal.is_bus_day(&saturday));  // Saturday
    }

    #[test]
    fn test_is_non_business_day() {
        let cal = fixture_hol_cal();
        let hol = NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let no_hol = NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let saturday = NaiveDateTime::parse_from_str("2024-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(cal.is_non_bus_day(&hol));  // Monday in Hol list
        assert!(!cal.is_non_bus_day(&no_hol));  //Thursday
        assert!(cal.is_non_bus_day(&saturday));  // Saturday
    }

    #[test]
    fn test_next_bus_day() {
        let cal = fixture_hol_cal();
        let hol = NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = cal.next_bus_day(&hol);
        assert_eq!(next, NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());

        let sat = NaiveDateTime::parse_from_str("2015-09-05 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = cal.next_bus_day(&sat);
        assert_eq!(next, NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());

        let fri = NaiveDateTime::parse_from_str("2015-09-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = cal.next_bus_day(&fri);
        assert_eq!(next, NaiveDateTime::parse_from_str("2015-09-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
    }

    #[test]
    fn test_prev_bus_day() {
        let cal = fixture_hol_cal();
        let hol = NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let prev = cal.prev_bus_day(&hol);
        assert_eq!(prev, NaiveDateTime::parse_from_str("2015-09-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());

        let fri = NaiveDateTime::parse_from_str("2015-09-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = cal.prev_bus_day(&fri);
        assert_eq!(next, NaiveDateTime::parse_from_str("2015-09-04 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
    }

    #[test]
    fn test_adjust() {
        let cal = fixture_hol_cal();
        let non_bus = NaiveDateTime::parse_from_str("2024-03-30 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let res = adjust(&non_bus, &cal, &Modifier::F);
        assert_eq!(res, NaiveDateTime::parse_from_str("2024-04-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());

        let res = adjust(&non_bus, &cal, &Modifier::P);
        assert_eq!(res, NaiveDateTime::parse_from_str("2024-03-29 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());

        let res = adjust(&non_bus, &cal, &Modifier::ModF);
        assert_eq!(res, NaiveDateTime::parse_from_str("2024-03-29 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());

        let res = adjust(&non_bus, &cal, &Modifier::Act);
        assert_eq!(res, NaiveDateTime::parse_from_str("2024-03-30 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());

        let non_bus = NaiveDateTime::parse_from_str("2024-12-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let res = adjust(&non_bus, &cal, &Modifier::ModP);
        assert_eq!(res, NaiveDateTime::parse_from_str("2024-12-02 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());
    }

    fn fixture_hol_cal2() -> HolCal {
        let hols = vec![
            NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2015-09-09 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ];
        HolCal::new(hols, vec![5, 6])
    }

    #[test]
    fn test_union_cal() {
        let cal1 = fixture_hol_cal();
        let cal2 = fixture_hol_cal2();
        let ucal = UnionCal::new(vec![cal1, cal2], None);

        let sat = NaiveDateTime::parse_from_str("2015-09-05 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = ucal.next_bus_day(&sat);
        assert_eq!(next, NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());
    }

    #[test]
    fn test_union_cal_with_settle() {
        let hols = vec![
            NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2015-09-09 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ];
        let scal = HolCal::new(hols, vec![5, 6]);
        let holcal = HolCal::new(vec![], vec![5,6]);
        let cal = UnionCal::new(vec![holcal], vec![scal].into());


        let mon = NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = cal.next_bus_day(&mon);
        assert_eq!(next, NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap());
    }
}
