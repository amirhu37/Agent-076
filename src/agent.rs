#[allow(dead_code)]
#[allow(non_snake_case)]

use crate::{action::Action, predicate::Predicate};
use std::fmt::Display;
use std::{fmt::Formatter, usize};


use std::sync::atomic::{AtomicUsize, Ordering};
// use rand::Rng;


// Atomic counter for generating unique IDs
static COUNTER: AtomicUsize = AtomicUsize::new(1);

struct ID {
    number_id: usize,
}
impl ID {
    fn new(id : Option<usize>) -> Self {
        match id {
            Some(id ) => { Self { number_id: id } }
            None => {Self { number_id: COUNTER.fetch_add(1, Ordering::Relaxed)}}}
    }
    // fn len(&self) -> usize{
    //     self.number_id.to_string().len()
    // }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}", self.number_id)
    }
}

impl Default for ID {
    fn default() -> Self {
        Self::new(None)
    }
}


pub struct Agent{
    id : ID,
    name : String,
    belief : Predicate , 
    actions : Action ,
    goals : Predicate
}



impl Agent {
    pub fn new(id :  Option<usize> , name : String) -> Self{

        let Id: ID = match id {
            Some(id) =>{
                if id.to_string().len() != 5 {panic!("less that 5 digit");}
                ID::new(Some(id))
            }
            None => ID::default(),
            };
        Self{
            id : Id,
            name ,
            actions : Action::new(),
            belief : Predicate::default(),
            goals : Predicate::default(),
            }
        }
    pub fn add_action(&mut self , predicate : Predicate){
        self.actions.action_set.push(predicate);
        }
    pub fn check_actions(&self) -> bool{
        for action in self.actions.action_set.iter(){
            if action.clone().is_active(){
                return true ;}
                }
                false
        }
//     pub fn add_belief(&mut self , predicate : Predicate){
//         self.belief.push(predicate);
//     }
//     pub fn add_goal(&self, predicate : Predicate){
//         self.goals.push(predicate);
//         }
}

impl Display for Agent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Agent {{ id: {}, name: {}, \n
                    actions: {:?}, belief : {:?},\n
                    goals: {:?} }}", 
                    self.id, 
                    self.name,  
                    self.actions, 
                    self.belief, 
                    self.goals)}

    
}