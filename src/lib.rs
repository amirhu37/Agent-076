#[allow(non_snake_case)]
use std::process::Command;

use ndarray::{s, Array2};


pub mod agent;
pub mod predicate;
pub mod action;
pub mod state;
pub mod env;



pub fn datatype<T>(_: &T) -> &str {
    let rtn: &str = std::any::type_name::<T>();
    // let mut e: Vec<&str> = rtn.split("::").collect::<Vec<&str>>();
    // e.reverse();
    // e[0]
    rtn
    }




/// Clears the terminal screen. Only works on Windows.
pub fn cls() {
    Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .unwrap();
}


pub fn learn(
    learning_rate: f64,
    discount_factor: f64,
    reward: f64,
    action: usize, // Use usize for array indexing
    state: usize,  // Use usize for array indexing
    mut q_t: Array2<f64>,
) -> Array2<f64> {
    let lr = learning_rate;
    let gamma = discount_factor;

    // Find the maximum value in the row corresponding to `state`
    let max_q_value = q_t.slice(s![state, ..]).iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Update the Q-value for the given state and action
    q_t[[state, action]] = lr * (reward + gamma * max_q_value) + (1.0 - lr) * q_t[[state, action]];

    q_t
}
