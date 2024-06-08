//! Provides static data for pre-existing named holiday calendars.
//!

pub mod all;
pub mod bus;
pub mod nyc;
pub mod tgt;
pub mod ldn;
pub mod stk;
pub mod osl;
pub mod zur;
pub mod tro;
pub mod tyo;

use crate::calendars::calendar::Cal;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use pyo3::{PyErr};
use pyo3::exceptions::{PyValueError};

fn get_weekmask_by_name(name: &str) -> Result<Vec<u8>, PyErr> {
    let hmap: HashMap<&str, &[u8]> = HashMap::from([
        ("all", all::WEEKMASK),
        ("bus", bus::WEEKMASK),
        ("nyc", nyc::WEEKMASK),
        ("tgt", tgt::WEEKMASK),
        ("ldn", ldn::WEEKMASK),
        ("stk", stk::WEEKMASK),
        ("osl", osl::WEEKMASK),
        ("zur", zur::WEEKMASK),
        ("tro", tro::WEEKMASK),
        ("tyo", tyo::WEEKMASK)
    ]);
    match hmap.get(name) {
        None => Err(PyValueError::new_err(format!("'{}' is not found in list of existing calendars.", name))),
        Some(value) => Ok(value.to_vec())
    }
}

fn get_holidays_by_name(name: &str) -> Result<Vec<NaiveDateTime>, PyErr> {
    let hmap: HashMap<&str, &[&str]> = HashMap::from([
        ("all", all::HOLIDAYS),
        ("bus", bus::HOLIDAYS),
        ("nyc", nyc::HOLIDAYS),
        ("tgt", tgt::HOLIDAYS),
        ("ldn", ldn::HOLIDAYS),
        ("stk", stk::HOLIDAYS),
        ("osl", osl::HOLIDAYS),
        ("zur", zur::HOLIDAYS),
        ("tro", tro::HOLIDAYS),
        ("tyo", tyo::HOLIDAYS),
    ]);
    match hmap.get(name) {
        None => Err(PyValueError::new_err(format!("'{}' is not found in list of existing calendars.", name))),
        Some(value) => Ok(value.iter().map(|x| NaiveDateTime::parse_from_str(x, "%Y-%m-%d %H:%M:%S").unwrap()).collect())
    }
}

// fn get_rules_by_name(name: &str) -> Result<Vec<&str>, PyErr> {
//     let hmap: HashMap<&str, &[&str]> = HashMap::from([
//         ("all", all::RULES),
//         ("bus", bus::RULES),
//         ("nyc", nyc::RULES),
//         ("tgt", tgt::RULES),
//         ("ldn", ldn::RULES),
//         ("stk", stk::RULES),
//         ("osl", osl::RULES),
//         ("zur", zur::RULES),
//         ("tro", tro::RULES),
//         ("tyo", tyo::RULES),
//     ]);
//     match hmap.get(name) {
//         None => Err(PyValueError::new_err(format!("'{}' is not found in list of existing calendars.", name))),
//         Some(value) => Ok(value.to_vec())
//     }
// }

/// Return a static `Cal` specified by a named identifier.
///
/// For available 3-digit names see `named` module documentation.
///
/// # Examples
///
/// ```rust
/// let ldn_cal = get_calendar_by_name("ldn")?;
/// ```
pub fn get_calendar_by_name(name: &str) -> Result<Cal, PyErr> {
    Ok(Cal::new(
        get_holidays_by_name(name)?,
        get_weekmask_by_name(name)?,
        // get_rules_by_name(name)?
    ))
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendars::calendar::DateRoll;

    #[test]
    fn test_get_weekmask() {
        let result = get_weekmask_by_name("bus").unwrap();
        assert_eq!(result, vec![5,6]);
    }

    #[test]
    fn test_get_holidays() {
        let result = get_holidays_by_name("bus").unwrap();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_get_rules() {
        let result = get_rules_by_name("bus").unwrap();
        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_get_cal() {
        let result = get_calendar_by_name("bus").unwrap();
        let expected = Cal::new(vec![], vec![5, 6], vec![]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all() {
        let cal = get_calendar_by_name("all").unwrap();
        assert!(cal.is_bus_day(&NaiveDateTime::parse_from_str("2024-11-11 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_nyc() {
        let cal = get_calendar_by_name("nyc").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-11-11 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_tgt() {
        let cal = get_calendar_by_name("tgt").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-05-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_ldn() {
        let cal = get_calendar_by_name("ldn").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-08-26 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_stk() {
        let cal = get_calendar_by_name("stk").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-06-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_osl() {
        let cal = get_calendar_by_name("osl").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-05-17 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_zur() {
        let cal = get_calendar_by_name("zur").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-08-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_tro() {
        let cal = get_calendar_by_name("tro").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-09-30 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_tyo() {
        let cal = get_calendar_by_name("tyo").unwrap();
        assert!(cal.is_holiday(&NaiveDateTime::parse_from_str("2024-1-3 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()));
    }

    #[test]
    fn test_get_calendar_error() {
        match get_calendar_by_name("badname") {
            Ok(val) => assert!(false),
            Err(_) => assert!(true),
        }
    }

}