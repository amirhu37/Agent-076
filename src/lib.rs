#[allow(non_snake_case)]
use std::process::Command;


pub mod agent;
pub mod predicate;
pub mod action;
pub mod state;
// pub mod Predicate;



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


pub fn  mean(vec_2d : &Vec<Vec<f64>> ) -> f64{
    let total_sum: f64 = vec_2d.iter().flatten().sum();
    let count: usize = vec_2d.iter().map(|v| v.len()).sum();
    total_sum / count as f64
}