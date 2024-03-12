use indexmap::set::IndexSet;
use std::sync::Arc;
use ndarray::Array1;

#[derive(Clone, Default)]
pub struct Dual {
    real: f64,
    vars: Arc<IndexSet<String>>,
    dual: Array1<f64>,
}

#[derive(Clone)]
enum VarsState {
    EquivByArc,  // Duals share an Arc ptr to their Vars
    EquivByVal,  // Duals share the same vars in the same order but no Arc ptr
    Superset,    // The Dual vars contains all of the queried values and is larger set
    Subset,      // The Dual vars is contained in the queried values and is smaller set
    Difference,  // The Dual vars and the queried set contain different values.
}

impl Dual {
    pub fn new(real: f64, vars: Vec<String>, dual: Vec<f64>) -> Self {
        let unique_vars_ = Arc::new(IndexSet::from_iter(vars));
        let dual_ = if dual.is_empty() {Array1::ones(unique_vars_.len())} else {Array1::from_vec(dual)};
        assert!(unique_vars_.len() == dual_.len());
        Self {real, vars: unique_vars_, dual: dual_}
    }

    pub fn new_from(other: &Self, real: f64, vars: Vec<String>, dual: Vec<f64>) -> Self {
        let new = Self::new(real, vars, dual);
        new.to_new_vars(&other.vars, None)
    }

    pub fn ptr_eq(&self, other: &Dual) -> bool {
        Arc::ptr_eq(&self.vars, &other.vars)
    }

    fn vars_compare(&self, arc_vars: &Arc<IndexSet<String>>) -> VarsState {
        if Arc::ptr_eq(&self.vars, arc_vars) {
            VarsState::EquivByArc
        } else if self.vars.len() == arc_vars.len()
            && self.vars.iter().zip(arc_vars.iter()).all(|(a, b)| a == b) {
            VarsState::EquivByVal
        } else if self.vars.len() >= arc_vars.len()
            && arc_vars.iter().all(|var| self.vars.contains(var)) {
            VarsState::Superset
        } else if self.vars.len() < arc_vars.len()
            && self.vars.iter().all(|var| arc_vars.contains(var)) {
            VarsState::Subset
        } else {
            VarsState::Difference
        }
    }

    pub fn to_new_vars(&self, arc_vars: &Arc<IndexSet<String>>, state: Option<VarsState>) -> Self {
        let dual_: Array1<f64>;
        let match_val = state.unwrap_or_else(|| self.vars_compare(&arc_vars));
        match match_val {
            VarsState::EquivByArc | VarsState :: EquivByVal => dual_ = self.dual.clone(),
            _ => {
                let lookup_or_zero = |v| {
                    match self.vars.get_index_of(v) {
                        Some(idx) => self.dual[idx],
                        None => 0.0_f64,
                    }
                };
                dual_ = Array1::from_vec(arc_vars.iter().map(lookup_or_zero).collect());
            }
        }
        Self {real: self.real, vars: Arc::clone(arc_vars), dual: dual_}
    }

    pub fn to_union_vars(&self, other: &Self) -> (Self, Self) {
        let state = self.vars_compare(&other.vars);
        match state {
            VarsState::EquivByArc => (self.clone(), other.clone()),
            VarsState::EquivByVal => (self.clone(), other.to_new_vars(&self.vars, Some(state))),
            VarsState::Superset => (self.clone(), other.to_new_vars(&self.vars, Some(VarsState::Subset))),
            VarsState::Subset => (self.to_new_vars(&other.vars, Some(state)), other.clone()),
            VarsState::Difference => self.to_combined_vars(other, Some(state)),
        }
    }

    fn to_combined_vars(&self, other: &Self, state: Option<VarsState>) -> (Self, Self) {
        let comb_vars = Arc::new(IndexSet::from_iter(
            self.vars.union(&other.vars).map(|x| x.clone()),
        ));
        (self.to_new_vars(&comb_vars, state.clone()), other.to_new_vars(&comb_vars, state))
    }

}


// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn new() {
        let x = Dual::new(1.0, vec!["a".to_string(), "a".to_string()], Vec::new());
        assert_eq!(x.real, 1.0_f64);
        assert_eq!(*x.vars, IndexSet::<String>::from_iter(vec!["a".to_string()]));
        assert_eq!(x.dual, Array1::from_vec(vec![1.0_f64]));
    }

    #[test]
    fn new_with_dual() {
        let x = Dual::new(1.0, vec!["a".to_string(), "a".to_string()], vec![2.5]);
        assert_eq!(x.real, 1.0_f64);
        assert_eq!(*x.vars, IndexSet::<String>::from_iter(vec!["a".to_string()]));
        assert_eq!(x.dual, Array1::from_vec(vec![2.5_f64]));
    }

    #[test]
    #[should_panic]
    fn new_len_mismatch() {
        Dual::new(1.0, vec!["a".to_string(), "a".to_string()], vec![1.0, 2.0]);
    }

    #[test]
    fn ptr_eq() {
        let x = Dual::new(1.0, vec!["a".to_string()], vec![]);
        let y = Dual::new(1.0, vec!["a".to_string()], vec![]);
        assert!(x.ptr_eq(&y)==false);
    }

    #[test]
    fn to_new_vars() {
        let x = Dual::new(1.5, vec!["a".to_string(), "b".to_string()], vec![1., 2.]);
        let y = Dual::new(2.0, vec!["a".to_string(), "c".to_string()], vec![3., 3.]);
        let z = x.to_new_vars(&y.vars, None);
        assert_eq!(z.real, 1.5_f64);
        assert!(y.ptr_eq(&z));
        assert_eq!(z.dual, Array1::from_vec(vec![1.0, 0.0]));
    }

    #[test]
    fn new_from() {
        let x = Dual::new(1.5, vec!["a".to_string(), "b".to_string()], vec![1., 2.]);
        let y = Dual::new_from(&x, 2.0, vec!["a".to_string(), "c".to_string()], vec![3., 3.]);
        assert_eq!(y.real, 2.0_f64);
        assert!(y.ptr_eq(&x));
        assert_eq!(y.dual, Array1::from_vec(vec![3.0, 0.0]));
    }

    #[test]
    fn profile() {
        // Setup
        let x = Dual::new(1.0, vec!["a".to_string()], vec![]);
        let y = Dual::new(2.0, vec!["a".to_string()], vec![]);

        println!("Benchmarking Arc pointer comparison time.. \n");
        let now = Instant::now();
        // Code block to measure.
        {
            for _i in 0..100000000 {
                // Arc::ptr_eq(&x.vars, &y.vars);
                let x = 1.0_f64 * 2.0_f64 + 3.0_f64 - 1.0_f64 / 10.1_f64;
            }
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed / 100000000);
    }

}


