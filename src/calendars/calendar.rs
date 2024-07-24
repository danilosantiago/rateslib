//! Utilities to create business day calendars and perform financial date manipulation.
//!
//! ### Basic usage
//!
//! The `Cal` struct allows the construction of a single business day calendar, e.g.
//! a particular currency calendar. The below constructs two separate calendars,
//! one for some London holidays and
//! one for some Tokyo holidays in 2017.
//!
//! ```rust
//! // UK Monday 1st May Bank Holiday
//! let ldn = Cal::new(vec![ndt(2017, 5, 1)], vec![5, 6]);
//! // Japan Constitution Memorial Day, Greenery Day, Children's Day
//! let tky = Cal::new(vec![ndt(2017, 5, 3), ndt(2017, 5, 4), ndt(2017, 5, 5)], vec![5, 6]);
//! ```
//! These calendars are used to manipulate dates e.g.
//!
//! ```rust
//! let date = ndt(2017, 4, 28);  // Friday 28th April 2017
//! let spot = ldn.add_bus_days(&date, 2, true)?;
//! // Wednesday 3rd May 2017, observing the holiday.
//! ```
//!
//! ### Combination usage
//!
//! For use with multi-currency products calendars often need to be combined.
//!
//! ```rust
//! let ldn_tky = UnionCal::new(vec![ldn, tky], None);
//! let spot = ldn_tky.add_bus_days(&date, 2, true)?;
//! // Monday 8th May 2017, observing all holidays.
//! ```
//!
//! Particularly when adjusting for FX transaction calendars the non-USD calendars may be used
//! for date determination but the US calendar is used to validate eligible settlement.
//! This is also a union of calendars but it is enforced via the `settlement_calendars` field.
//!
//! ```rust
//! let tgt = Cal::new(vec![], vec![5, 6]);
//! let nyc = Cal::new(vec![ndt(2023, 6, 19)], vec![5, 6]);  // Juneteenth Holiday
//! let tgt__nyc = UnionCal::new(vec![tgt], vec![nyc].into());
//! ```
//!
//! The spot (T+2) date as measured from Friday 16th June 2023 ignores the US calendar for date
//! determination and allows Tuesday 20th June 2023 since the US holiday is on the Monday.
//!
//! ```rust
//! let date = ndt(2023, 6, 16);  // Friday 16th June 2023
//! let spot = tgt__nyc.add_bus_days(&date, 2, true)?;
//! // Tuesday 20th June 2023, ignoring the US holiday on Monday.
//! ```
//!
//! On the other hand as measured from Thursday 15th June 2023 the spot cannot be on the Monday
//! when `settlement` is enforced over the US calendar.
//!
//! ```rust
//! let date = ndt(2023, 6, 15);  // Thursday 15th June 2023
//! let spot = tgt__nyc.add_bus_days(&date, 2, true)?;
//! // Tuesday 20th June 2023, enforcing no settlement on US holiday.
//! ```
//!
//! If `settlement` is not enforced spot can be set as the Monday for this calendar, since it is
//! not a European holiday.
//!
//! ```rust
//! let spot = tgt__nyc.add_bus_days(&date, 2, false)?;
//! // Monday 19th June 2023, ignoring the US holiday settlement requirement.
//! ```

use chrono::prelude::*;
use chrono::Weekday;
use indexmap::set::IndexSet;
use pyo3::exceptions::{PyValueError};
use pyo3::{pyclass, PyErr};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub use crate::calendars::dateroll::{get_imm, get_roll, DateRoll, Modifier, RollDay};
use crate::calendars::named::get_calendar_by_name;

/// A business day calendar with a singular list of holidays.
///
/// A business day calendar is formed of 2 components:
///
/// - `week_mask`: which defines the days of the week that are not general business days. In Western culture these
///   are typically `[5, 6]` for Saturday and Sunday.
/// - `holidays`: which defines specific dates that may be exceptions to the general working week, and cannot be
///   business days.
///
#[pyclass(module = "rateslib.rs")]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cal {
    pub(crate) holidays: IndexSet<NaiveDateTime>,
    pub(crate) week_mask: HashSet<Weekday>,
    // pub(crate) meta: Vec<String>,
}

impl Cal {
    /// Create a calendar.
    ///
    /// `holidays` provide a vector of dates that cannot be business days. `week_mask` is a vector of days
    /// (0=Mon,.., 6=Sun) that are excluded from the working week.
    pub fn new(
        holidays: Vec<NaiveDateTime>,
        week_mask: Vec<u8>,
        // rules: Vec<&str>
    ) -> Self {
        Cal {
            holidays: IndexSet::from_iter(holidays),
            week_mask: HashSet::from_iter(
                week_mask.into_iter().map(|v| Weekday::try_from(v).unwrap()),
            ),
            // meta: rules.into_iter().map(|x| x.to_string()).collect(),
        }
    }
}

/// A business day calendar which is the potential union of multiple calendars,
/// with the additional constraint of also ensuring settlement compliance with one or more
/// other calendars.
///
/// When the union of a business day calendar is observed the following are true:
///
/// - a weekday is such if it is a weekday in all calendars.
/// - a holiday is such if it is a holiday in any calendar.
/// - a business day is such if it is a business day in all calendars.
///
/// A business day is defined as allowing settlement relative to an associated calendar if:
///
/// - the date in question is also a business day in the associated settlement calendar.
#[pyclass(module = "rateslib.rs")]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct UnionCal {
    pub(crate) calendars: Vec<Cal>,
    pub(crate) settlement_calendars: Option<Vec<Cal>>,
}

impl UnionCal {
    pub fn new(calendars: Vec<Cal>, settlement_calendars: Option<Vec<Cal>>) -> Self {
        UnionCal {
            calendars,
            settlement_calendars,
        }
    }
}

/// A wrapper for a UnionCal struct specified by a string representation.
///
/// This struct is designed for use when serialization of a calendar as part of an another composite
/// struct seeks to be related to named calendar combinations and not an inefficient list of dates.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "NamedCalDataModel")]
pub struct NamedCal {
    name: String,
    #[serde(skip)]
    union_cal: UnionCal,
}

#[derive(Deserialize)]
struct NamedCalDataModel {
    name: String,
}

impl std::convert::From<NamedCalDataModel> for NamedCal {
    fn from(model: NamedCalDataModel) -> Self {
        Self::try_new(&model.name).expect("NamedCal data model contains bad data.")
    }
}

impl NamedCal {
    fn try_new(name: &str) -> Result<Self, PyErr> {
        let parts: Vec<&str> = name.split("|").collect();
        if parts.len() > 2 {
            Err(PyValueError::new_err("Cannot use more than one pipe ('|') operator in `name`."))
        }
        else if parts.len() == 1 {
            let cals: Vec<Cal> = parse_cals(parts[0])?;
            Ok(Self {
                name: name.to_string(),
                union_cal: UnionCal {calendars: cals, settlement_calendars: None}
            })
        } else {
            let cals: Vec<Cal> = parse_cals(parts[0])?;
            let settle_cals: Vec<Cal> = parse_cals(parts[1])?;
            Ok(Self {
                name: name.to_string(),
                union_cal: UnionCal {calendars: cals, settlement_calendars: Some(settle_cals)}
            })
        }
    }
}

fn parse_cals(name: &str) -> Result<Vec<Cal>, PyErr> {
    let mut cals: Vec<Cal> = Vec::new();
    for cal in name.split(",") {
        cals.push(get_calendar_by_name(cal)?)
    }
    Ok(cals)
}

impl DateRoll for Cal {
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
        self.settlement_calendars
            .as_ref()
            .map_or(true, |v| !v.iter().any(|cal| cal.is_non_bus_day(date)))
    }
}

impl DateRoll for NamedCal {
    fn is_weekday(&self, date: &NaiveDateTime) -> bool {
        self.union_cal.is_weekday(date)
    }

    fn is_holiday(&self, date: &NaiveDateTime) -> bool {
        self.union_cal.is_holiday(date)
    }

    fn is_settlement(&self, date: &NaiveDateTime) -> bool {
        self.union_cal.is_settlement(date)
    }
}

impl<T> PartialEq<T> for UnionCal
where
    T: DateRoll,
{
    fn eq(&self, other: &T) -> bool {
        let cd1 = self
            .cal_date_range(&ndt(1970, 1, 1), &ndt(2200, 12, 31))
            .unwrap();
        let cd2 = other
            .cal_date_range(&ndt(1970, 1, 1), &ndt(2200, 12, 31))
            .unwrap();
        cd1.iter().zip(cd2.iter()).all(|(x, y)| {
            self.is_bus_day(x) == other.is_bus_day(x)
                && self.is_settlement(x) == other.is_settlement(y)
        })
    }
}

impl PartialEq<UnionCal> for Cal {
    fn eq(&self, other: &UnionCal) -> bool {
        let cd1 = self
            .cal_date_range(&ndt(1970, 1, 1), &ndt(2200, 12, 31))
            .unwrap();
        let cd2 = other
            .cal_date_range(&ndt(1970, 1, 1), &ndt(2200, 12, 31))
            .unwrap();
        cd1.iter().zip(cd2.iter()).all(|(x, y)| {
            self.is_bus_day(x) == other.is_bus_day(x)
                && self.is_settlement(x) == other.is_settlement(y)
        })
    }
}

/// Create a `NaiveDateTime` with default null time.
///
/// Panics if date values are invalid.
pub fn ndt(year: i32, month: u32, day: u32) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(year, month, day)
        .expect("`year`, `month` `day` are invalid.")
        .and_hms_opt(0, 0, 0)
        .unwrap()
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendars::named::get_calendar_by_name;

    fn fixture_hol_cal() -> Cal {
        let hols = vec![ndt(2015, 9, 5), ndt(2015, 9, 7)]; // Saturday and Monday
        Cal::new(hols, vec![5, 6])
    }

    #[test]
    fn test_is_holiday() {
        let cal = fixture_hol_cal();
        let hol =
            NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let no_hol =
            NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let saturday =
            NaiveDateTime::parse_from_str("2024-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(cal.is_holiday(&hol)); // In hol list
        assert!(!cal.is_holiday(&no_hol)); // Not in hol list
        assert!(!cal.is_holiday(&saturday)); // Not in hol list
    }

    #[test]
    fn test_is_weekday() {
        let cal = fixture_hol_cal();
        let hol =
            NaiveDateTime::parse_from_str("2015-09-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let no_hol =
            NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let saturday =
            NaiveDateTime::parse_from_str("2024-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let sunday =
            NaiveDateTime::parse_from_str("2024-01-07 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(cal.is_weekday(&hol)); // Monday
        assert!(cal.is_weekday(&no_hol)); //Thursday
        assert!(!cal.is_weekday(&saturday)); // Saturday
        assert!(!cal.is_weekday(&sunday)); // Sunday
    }

    fn fixture_hol_cal2() -> Cal {
        let hols = vec![
            NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2015-09-09 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ];
        Cal::new(hols, vec![5, 6])
    }

    #[test]
    fn test_union_cal() {
        let cal1 = fixture_hol_cal();
        let cal2 = fixture_hol_cal2();
        let ucal = UnionCal::new(vec![cal1, cal2], None);

        let sat =
            NaiveDateTime::parse_from_str("2015-09-05 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = ucal.roll_forward_bus_day(&sat);
        assert_eq!(
            next,
            NaiveDateTime::parse_from_str("2015-09-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[test]
    fn test_union_cal_with_settle() {
        let hols = vec![
            NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2015-09-09 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ];
        let scal = Cal::new(hols, vec![5, 6]);
        let cal = Cal::new(vec![], vec![5, 6]);
        let ucal = UnionCal::new(vec![cal], vec![scal].into());

        let mon =
            NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let next = ucal.roll_forward_bus_day(&mon);
        assert_eq!(
            next,
            NaiveDateTime::parse_from_str("2015-09-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[test]
    fn test_named_cal() {
        let ncal = NamedCal::try_new("tgt,nyc").unwrap();

        assert!(ncal.is_non_bus_day(&ndt(1970, 2, 16)));  // NYC holiday
        assert!(ncal.is_non_bus_day(&ndt(1970, 5, 1)));  // TGT holiday
        assert!(ncal.is_bus_day(&ndt(1970, 2, 17)));
    }

    #[test]
    fn test_named_cal_pipe() {
        let ncal = NamedCal::try_new("tgt,nyc|ldn").unwrap();

        assert!(ncal.is_non_bus_day(&ndt(1970, 2, 16)));  // NYC holiday
        assert!(ncal.is_non_bus_day(&ndt(1970, 5, 1)));  // TGT holiday
        assert!(ncal.is_bus_day(&ndt(1970, 2, 17)));

        assert!(!ncal.is_settlement(&ndt(1970, 5, 4)));  // LDN holiday
        assert!(ncal.is_settlement(&ndt(1970, 5, 1)));  // not LDN holiday
    }

    #[test]
    fn test_docstring() {
        let ldn = Cal::new(vec![ndt(2017, 5, 1)], vec![5, 6]); // UK Monday 1st May Bank Holiday
        let tky = Cal::new(
            vec![ndt(2017, 5, 3), ndt(2017, 5, 4), ndt(2017, 5, 5)],
            vec![5, 6],
        );

        let date = ndt(2017, 4, 28); // Friday 28th April 2017
        let spot = ldn.add_bus_days(&date, 2, true).unwrap();
        assert_eq!(spot, ndt(2017, 5, 3));

        let ldn_tky = UnionCal::new(vec![ldn, tky], None);
        let spot = ldn_tky.add_bus_days(&date, 2, true).unwrap();
        assert_eq!(spot, ndt(2017, 5, 8));

        let tgt = Cal::new(vec![], vec![5, 6]);
        let nyc = Cal::new(vec![ndt(2023, 6, 19)], vec![5, 6]); // Juneteenth Holiday
        let tgt_nyc = UnionCal::new(vec![tgt], vec![nyc].into());

        let date = ndt(2023, 6, 16);
        let spot = tgt_nyc.add_bus_days(&date, 2, true).unwrap();
        assert_eq!(spot, ndt(2023, 6, 20));

        let date = ndt(2023, 6, 15);
        let spot = tgt_nyc.add_bus_days(&date, 2, true).unwrap();
        assert_eq!(spot, ndt(2023, 6, 20));

        let spot = tgt_nyc.add_bus_days(&date, 2, false).unwrap();
        assert_eq!(spot, ndt(2023, 6, 19));
    }

    #[test]
    fn test_cross_equality() {
        let cal = fixture_hol_cal();
        let ucal = UnionCal::new(vec![cal.clone()], None);
        assert_eq!(cal, ucal);
        assert_eq!(ucal, cal);

        let ucals = UnionCal::new(vec![cal.clone()], vec![cal.clone()].into());
        assert_ne!(cal, ucals);
        assert_ne!(ucals, cal);

        let cal2 = fixture_hol_cal2();
        assert_ne!(cal2, ucal);
        assert_ne!(ucal, cal2);
    }
}
