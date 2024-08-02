use crate::dual::{FieldOps, MathFuncs, DualsOrF64};
use crate::calendars::{CalType, Convention};
use std::ops::Mul;
use chrono::NaiveDateTime;
use crate::curves::{CurveInterpolation};
use crate::curves::nodes::NodesTimestamp;
use crate::curves::interpolation::utils::linear_interp;

pub struct LinearInterpolator {
    calendar: CalType,
    convention: Convention,
}

impl LinearInterpolator {
    pub fn new(calendar: CalType, convention: Convention) -> Self {
        LinearInterpolator {calendar, convention}
    }
}

impl CurveInterpolation for LinearInterpolator {
    fn interpolated_value(&self, nodes: &NodesTimestamp, date: &NaiveDateTime) -> DualsOrF64 {
        let x = date.and_utc().timestamp();
        let index = self.node_index(nodes, x);

        macro_rules! interp {
            ($Variant: ident, $indexmap: expr) => {{
                let (x1, y1) = $indexmap.get_index(index).unwrap();
                let (x2, y2) = $indexmap.get_index(index + 1_usize).unwrap();
                DualsOrF64::$Variant(linear_interp(*x1 as f64, y1, *x2 as f64, y2, x as f64))
            }}
        }
        match nodes {
            NodesTimestamp::F64(m) => interp!(F64, m),
            NodesTimestamp::Dual(m) => interp!(Dual, m),
            NodesTimestamp::Dual2(m) => interp!(Dual2, m),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use crate::curves::nodes::Nodes;
    use crate::calendars::{NamedCal, ndt};
    use crate::dual::Dual;

    fn nodes_timestamp_fixture() -> NodesTimestamp {
        let nodes = Nodes::F64(IndexMap::from_iter(vec![
            (ndt(2000, 1, 1), 1.0_f64),
            (ndt(2001, 1, 1), 0.99_f64),
            (ndt(2002, 1, 1), 0.98_f64),
        ]));
        NodesTimestamp::from(nodes)
    }

    #[test]
    fn test_linear() {
        let nts = nodes_timestamp_fixture();
        let li = LinearInterpolator {
            calendar: CalType::NamedCal(NamedCal::try_new("all").unwrap()),
            convention: Convention::Act365F,
        };
        let result = li.interpolated_value(&nts, &ndt(2000, 7, 1));
        // expected = 1.0 + (182 / 366) * (0.99 - 1.0) = 0.995027
        assert_eq!(result, DualsOrF64::F64(0.9950273224043715));
    }
}
