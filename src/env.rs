use std::collections::HashMap;

// use derive_more::Display as cDisplay;
use ndarray::Array1;

use crate::agent::{Agent, MyAgent};


pub trait Env{
    fn new(
        id: Option<usize>,
        name: String,
        action_space: Array1<i32>,
        observation_space: Option<Vec<f64>>,
        // state: Option<f64>
    ) -> Result<Self, String> where Self: Sized;
    
    fn reset(&mut self ) ->();
    fn step(&mut self , action : i32, state : f64 ) -> (f64 , f64, bool, HashMap<String, f64>);
    fn close(&mut self) -> ();

}

impl std::fmt::Display for dyn Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnvTrait")
        }
    
}




// #[derive(Clone, Debug, cDisplay,Default)]
// #[display(fmt = "Agent(Id:{id:?}, Name: {name}, action space: {action_space:?},  observation  space: {observation_space:?}, state: {state:?})")]
#[derive(Clone, Debug)]
pub struct MyEnv {
    pub id: Option<usize>,
    pub name: String,
    pub action_space: Array1<i32>,
    pub observation_space: Option<Vec<f64>>,
    // pub state: Option<f64>
}

impl Env for MyEnv {
    fn new(
        id: Option<usize>,
        name: String,
        action_space: Array1<i32>,
        observation_space: Option<Vec<f64>>,
        // state: Option<f64>
    ) -> Result<Self, String> 
    where Self: Sized {
        Ok(MyEnv {
            id,
            name,
            action_space,
            observation_space,
            // state 
        })
    }


    fn reset(&mut self ) ->() {
        todo!()
    }
    fn step(&mut self , action : i32 , state : f64) ->(f64, f64, bool, HashMap<String, f64>) {
        let rand: f64 = rand::random();

        // self.state = Some(2.);
        let reward: f64 = <MyAgent as Agent>::policy(action, state);
        let done: bool = rand >= 0.95 ;
        let info: HashMap<String, f64> = HashMap::new();
        (state, reward, done, info)
        }
    fn close(&mut self)->(){}
}