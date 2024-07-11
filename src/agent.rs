use derive_more::Display as cDisplay;
use ndarray::{array, Array1, Array2};
use ndarray_rand::RandomExt;

use std::ops::Add;
use std::usize;
use std::sync::atomic::{AtomicUsize, Ordering};

// use rand::prelude::*;
use rand::distributions::Uniform;

// use crate::action;


// Atomic counter for generating unique IDs
static COUNTER: AtomicUsize = AtomicUsize::new(1);


#[derive(Clone, Debug, cDisplay,Default)]
struct ID {
    #[display("ID({_0})")]
    number_id: usize,
}

impl ID {
    fn new(id : Option<usize>) -> Self {
        match id {
            Some(id ) => { Self { number_id: id } }
            None => {Self { number_id: COUNTER.fetch_add(1, Ordering::Relaxed)}}}
    }
}


pub trait Agent{
    fn new( id :  Option<usize> ,
        name : String ,
        actions : Option<Array1<i32>>,
        q_tabel : Option<Array2<f64>>,
        utility :  Option<f64>
    ) -> Result<Self, String> where Self: Sized;
    fn add_action(&mut self , action : i32) ->();
    fn get_action(&mut self ) -> Array1<i32>;
    fn policy(action : i32 , observation : f64 ) -> f64;
    fn q_learning(learning_rate: f64, discount_factor: f64) -> Array2<f64>;


}







/// Test codes
/// ------------
/// #######################
/// ###                 ###
/// ###                 ###
/// ###                 ###
/// #######################
#[derive(cDisplay)]
#[display(fmt = "Agent(Id:{id}, Name: {name}, Actions: {actions:?},  Utility: {q_tabel:?})")]
pub struct MyAgent{
    #[display("ID({_0})")]
    id : ID,
    #[display("name {_0}")]
    name : String,
    #[display("Action {_0}")]
    pub actions : Array1<i32>  ,
    #[display("q_tabel {_0}")]
    pub q_tabel :  Array2<f64>,
    #[display("utility {_0}")]
    pub utility : Option<f64>

}


impl  Agent for MyAgent {
    fn new( id :  Option<usize> ,
                name : String ,
                actions : Option<Array1<i32>>,
                q_tabel : Option<Array2<f64>>,
                utility : Option<f64>,
            ) -> Result<Self, String>{
        let current = COUNTER.load(Ordering::SeqCst);
        let id: ID = match id {
            Some(id) =>{
                if id.to_string().len() >= 5 {return Err("Agent ID must not be More than 5 digit".to_string())}
                if id < current {return Err("Id Must be unique, curent id : ".to_string() + &id.to_string() )}
                ID::new(Some(id))
            }
            None => ID::default()};
    // Generating a random 2D array
        let random_matrix = Array2::random((2, 2), Uniform::new(0., 1.));
        Ok(
        Self{
            id : id,
            name ,
            actions : actions.unwrap_or(array![0]),
            q_tabel : q_tabel.unwrap_or(random_matrix),
            utility : Some(utility.unwrap_or(0.0))
            })
        }
    fn add_action(&mut self , action : i32){
        let _ = &self.actions.clone().add( action);
        }
        
    fn get_action(&mut self )-> Array1<i32>{
            self.actions.clone()
            }
        
    fn policy(action : i32 , observation : f64 ) -> f64 {
   // Flatten the 2D array into a 1D array
        // let flat_array = self.actions.as_slice().unwrap();

        // Create a random number generator
        // let mut rng = rand::thread_rng();

        // Generate a random index within the bounds of the flattened array
        // let random_index = rng.gen_range(0..flat_array.len());

        // Select the value at the random index
        // let x = flat_array[random_index] as f64;
        // observation * action as f64 + 0.2
        match (observation, action) {
            (observation, 0) if observation >  1. => 0.50,
            (observation, 0) if observation <  1. => 0.60,
            (observation, 1) if observation >  1. => 0.70,
            (observation, 2) if observation <  1. => -0.90,
            (observation, 1) if observation >= 1. => 0.01,
            _ => 0.0,
        }
        
    }
    
    fn q_learning(learning_rate: f64, discount_factor: f64) -> Array2<f64> {
        let _ = learning_rate;
        let _ = discount_factor;
        todo!()
    }

}



