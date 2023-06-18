extern crate ndarray;

use std::ops;
use ndarray::{Array1, Array, arr1, arr2, array};
// use ndarray_einsum_beta::*;
use num_traits;
// use indexmap::indexset;
use indexmap::set::IndexSet;

fn is_close(a: &f64, b: &f64, abs_tol: Option<f64>) -> bool {
    // used rather than equality for float numbers
    return (a-b).abs() < abs_tol.unwrap_or(1e-8)
}

#[derive(Clone, Debug)]
pub struct Dual {
    real : f64,
    vars : IndexSet<String>,
    dual : Array1<f64>,
}

impl Dual {
    pub fn new(real: f64, vars: &[&str], dual: &[f64]) -> Dual {
        let new_dual;
        if dual.len() != 0 && vars.len() != dual.len() {
            panic!("`dual` must have same length as `vars` or have zero length.")
        } else if dual.len() == 0 && vars.len() > 0 {
            new_dual = Array::ones(vars.len());
        } else {
            new_dual = arr1(&dual);
        }
        Dual{
            real: real,
            vars: IndexSet::from_iter(vars.iter().map(|x| x.to_string())),
            dual: new_dual,
        }
    }

    // pub fn pow(&self, arg: f64) -> Dual {
    //     return Dual {
    //         real: self.real.pow(arg),
    //         vars: self.vars,
    //         dual: Array1::from_iter()
    //     }
    // }

    fn upcast_combined(self, other: Dual) -> (Dual, Dual) {
        // check the set of vars in each Dual are equivalent
        if self.vars_check(&other) {
            // if the same vars then reproduce lhs and upcast rhs (in case of order differences)
            (self.clone(), other.to_new_vars(&self.vars))
        } else {
            // upcast both Duals to have same ordered vars
            let comb_vars = IndexSet::from_iter(self.vars.union(&other.vars).map(|x| x.clone()));
            (self.to_new_vars(&comb_vars), other.to_new_vars(&comb_vars))
        }
    }

    fn to_new_vars(&self, new_vars: &IndexSet<String>) -> Dual {
        // Take a Dual and redefine its derivatives according to a new set of variable tags.

        if self.vars.len() == new_vars.len() && self.vars.iter().zip(new_vars.iter()).filter(|&(a, b)| a == b).count() == self.vars.len()
        {   // new_vars equals vars in order and value so no need to redefine;
            return self.clone();
        } else {
            //
            let real = self.real.clone();
            let mut dual = Array::zeros(new_vars.len());
            for (i, var) in new_vars.iter().enumerate() {
                let index = self.vars.get_index_of(var);
                match index {
                    Some(value) => {
                        dual[[i]] = self.dual[[value]]
                    }
                    None => {}
                }
            }
            Dual {vars: new_vars.clone(), real, dual}
        }
    }

    fn vars_check(&self, other: &Dual) -> bool {
        // test if the vars of a Dual have the same elements but possibly a different order
        return self.vars.len() == other.vars.len() && self.vars.intersection(&other.vars).count() == self.vars.len()
    }
}

impl ops::Add<f64> for Dual {
    type Output = Dual;
    fn add(self, other: f64) -> Dual {
        Dual {vars: self.vars, real: self.real + other, dual: self.dual}
    }
}

impl ops::Sub<f64> for Dual {
    type Output = Dual;
    fn sub(self, other: f64) -> Dual {
        Dual {vars: self.vars, real: self.real - other, dual: self.dual}
    }
}

impl ops::Mul<f64> for Dual {
    type Output = Dual;
    fn mul(self, other: f64) -> Dual {
        Dual {vars: self.vars, real: self.real * other, dual: self.dual * other}
    }
}

impl PartialEq<f64> for Dual {
    fn eq(&self, other: &f64) -> bool {
        let d = Dual::new(*other, &[], &[]);
        return d == *self;
    }
}

impl ops::Neg for Dual {
    type Output = Dual;
    fn neg(self) -> Dual {
        Dual {vars: self.vars, real: -self.real, dual: -self.dual}
    }
}

impl std::iter::Sum for Dual {
    fn sum<I>(iter: I) -> Self
       where I: Iterator<Item = Dual>
    {
        return iter.fold(Dual::new(0.0, &[], &[]), |acc, x| acc + x)
        // let mut d = Dual::new(0.0, &[], &[]);
        // for val in iter { d = d + val }
        // return d
    }
}

impl ops::Add<Dual> for Dual {
    type Output = Dual;
    fn add(self, other: Dual) -> Dual {
        let (x, y) = self.upcast_combined(other);
        Dual {
            real: x.real + y.real,
            dual: x.dual + y.dual,
            vars: x.vars,
        }
    }
}

impl ops::Sub<Dual> for Dual {
    type Output = Dual;
    fn sub(self, other: Dual) -> Dual {
        let (x, y) = self.upcast_combined(other);
        Dual {
            real: x.real - y.real,
            dual: x.dual - y.dual,
            vars: x.vars
        }
    }
}

impl ops::Mul<Dual> for Dual {
    type Output = Dual;
    fn mul(self, other: Dual) -> Dual {
        let (x, y) = self.upcast_combined(other);
        Dual {
            real: x.real * y.real,
            dual: x.dual * y.real + y.dual * x.real,
            vars: x.vars
        }
    }
}

impl num_traits::identities::One for Dual {
    fn one() -> Dual {
        return Dual::new(1.0, &[], &[])
    }
}

impl num_traits::identities::Zero for Dual {
    fn zero() -> Dual {
        return Dual::new(0.0, &[], &[])
    }

    fn is_zero(&self) -> bool {
        return *self == Dual::new(0.0, &[], &[])
    }
}

impl PartialEq<Dual> for Dual {
    fn eq(&self, other: &Dual) -> bool {
        if self.real != other.real {
            return false
        }
        if ! self.vars_check(&other) {
            return false
        }
        let another = other.to_new_vars(&self.vars);
        for (i, elem) in self.dual.iter().enumerate() {
            if ! is_close(&another.dual[[i]], &elem, None) {
                return false
            }
        }
        return true
    }
}

fn arr1_dot(a1: Array1<Dual>, a2: Array1<Dual>) -> Dual {
    // Consumes two one dimensional arrays and produces a scalar value of their dot product.
    let z = a1.into_iter().zip(a2.into_iter()).map(|(x, y)| x * y).collect::<Vec<Dual>>();
    return z.into_iter().sum::<Dual>()
}

fn main() {
    let d2 = Dual::new(5.0, &["b", "c"], &[3.0, 1.0]);
    let d1 = Dual::new(5.0, &["c", "b"], &[1.0, 3.0]);

    println!("{}", d2 == d1);
    // let m1 = arr1(&[1, 2]);
    // let m2 = arr2(&[[1, 2], [3, 4]]);
    // println!("{:?}", einsum("i,ij->j", &[&m1, &m2]));
    //
    let z = arr1(&[d2.clone(), d2.clone()]);
    let b = arr1(&[d1, d2]);

    println!("{:?}", arr1_dot(z.clone(), b.clone()))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn add_dual_float {
//         //
//     }
// }