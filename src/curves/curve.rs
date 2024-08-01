
use crate::curves::nodes::{Nodes, NodesTimestamp};
use crate::curves::{CurveInterpolator, LogLinearInterpolator};
use crate::dual::{DualsOrF64, ADOrder, Dual, Dual2, set_order, get_variable_tags};
use chrono::NaiveDateTime;
use pyo3::PyErr;
use std::collections::HashMap;

use crate::json::JSON;
use serde::{Serialize, Deserialize};

///
pub struct Curve {
    nodes: NodesTimestamp,
    interpolator: CurveInterpolator,
    id: String,
}

impl Curve {
    pub fn try_new(
        nodes: Nodes,
        interpolator: CurveInterpolator,
        id: &str,
    ) -> Result<Self, PyErr> {
        let mut nodes = NodesTimestamp::from(nodes);
        nodes.sort_keys();
        Ok( Self {nodes, interpolator, id: id.to_string()} )
    }

    /// Get the `ADOrder` of the `Curve`.
    pub fn ad(&self) -> ADOrder {
        match self.nodes {
            NodesTimestamp::F64(_) => ADOrder::Zero,
            NodesTimestamp::Dual(_) => ADOrder::One,
            NodesTimestamp::Dual2(_) => ADOrder::Two,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendars::{CalType, NamedCal, Convention};
    use indexmap::IndexMap;
    use crate::calendars::ndt;

    fn curve_fixture() -> Curve {
        let nodes = Nodes::F64(IndexMap::from_iter(vec![
            (ndt(2000, 1, 1), 1.0_f64),
            (ndt(2001, 1, 1), 0.99_f64),
            (ndt(2002, 1, 1), 0.98_f64),
        ]));
        let interpolator = CurveInterpolator::LogLinear(
            LogLinearInterpolator::new(CalType::NamedCal(NamedCal::try_new("bus").unwrap()), Convention::Act365F)
        );
        Curve::try_new(nodes, interpolator, "crv").unwrap()
    }

    #[test]
    fn test_get_index() {
        let c = curve_fixture();
        let result = c.node_index(&ndt(2001, 7, 30));
        assert_eq!(result, 1_usize)
    }
//
//     #[test]
//     fn test_get_value() {
//         let c = curve_fixture();
//         let result = c.get_value(&ndt(2001, 7, 30));
//         assert_eq!(result, DualsOrF64::F64(0.985_f64))
//     }
}