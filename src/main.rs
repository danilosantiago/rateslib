// extern crate ndarray;
// extern crate indexmap;
// extern crate rateslibrs;
// use nalgebra as na;
use std::sync::Arc;

pub mod dual;
use dual::dual1::Dual;
use dual::Duals;
use indexmap::set::IndexSet;
use ndarray::Array;

fn dual_add_bm(a: &Dual, b: &Dual) -> Dual {
    a + b
}
// fn duals_add<T> (lhs: &T, rhs: &T) -> T {
//     lhs + rhs
// }

fn main() {
    let dual_ = Array::ones(2);
    let vars = IndexSet::from_iter((0..2).map(|x| format!("v{}", x).to_string()));
    let dual_2 = Array::ones(3);
    let vars2 = IndexSet::from_iter((0..3).map(|x| format!("v{}", x).to_string()));
    let a = Dual { real: 2.0, vars: Arc::new(vars), dual: dual_ };
    let b = Dual { real: 3.0, vars: Arc::new(vars2), dual: dual_2 };
    // for i in 1..50000 {
    //     dual_add_bm(&a, &b);
    // }
    let y = dual_add_bm(&a, &b);
    println!("{:?}", y);

    // let d2 = Dual::new(5.0, &["b", "c"], &[3.0, 1.0]);
    // let d1 = Dual::new(5.0, &["c", "b"], &[1.0, 3.0]);
    //
    // let x = na::Vector2::new(d1.clone(), d2.clone());
    //
    // let y = na::Vector2::new(d1, d2);
    //
    // println!("vector: {:?}", y.dot(&x))
    //
    // println!("{}", d2 == d1);
    // // let m1 = arr1(&[1, 2]);
    // // let m2 = arr2(&[[1, 2], [3, 4]]);
    // // println!("{:?}", einsum("i,ij->j", &[&m1, &m2]));
    // //
    // let z = arr1(&[d2.clone(), d2.clone()]);
    // let b = arr1(&[d1, d2]);
    //
    // println!("{:?}", arr1_dot(z.clone(), b.clone()))
}
