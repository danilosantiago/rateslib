use crate::json::JSON;
use crate::calendars::calendar::{Cal, UnionCal};

impl JSON for Cal {}
impl JSON for UnionCal {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendars::calendar::ndt;

    #[test]
    fn test_cal_json() {
        let hols = vec![ndt(2015, 9, 8), ndt(2015, 9, 10)];
        let hcal = Cal::new(hols, vec![5, 6]);
        let js = hcal.to_json().unwrap();
        let hcal2 = Cal::from_json(&js).unwrap();
        assert_eq!(hcal, hcal2);
    }

    #[test]
    fn test_union_cal_json() {
        let hols = vec![ndt(2015, 9, 8), ndt(2015, 9, 10)];
        let settle = vec![ndt(2015, 9, 11)];
        let hcal = Cal::new(hols, vec![5, 6]);
        let scal = Cal::new(settle, vec![5, 6]);
        let ucal = UnionCal::new(vec![hcal], vec![scal].into());
        let js = ucal.to_json().unwrap();
        let ucal2 = UnionCal::from_json(&js).unwrap();
        assert_eq!(ucal, ucal2);
    }
}
