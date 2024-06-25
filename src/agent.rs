use syn::token::Fn;

#[allow(dead_code)]
#[allow(non_snake_case)]

use crate::{action::*, predicate::*};
use std::any::Any;
use std::borrow::Borrow;
use std::fmt::{Arguments, Display};
use std::{fmt::Formatter, usize};


use std::sync::atomic::{AtomicUsize, Ordering};
// Atomic counter for generating unique IDs
static COUNTER: AtomicUsize = AtomicUsize::new(1);


#[derive(Clone)]
struct ID {number_id: usize,}
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
        write!(f, "{}", self.number_id)}}
impl Default for ID {
    fn default() -> Self {
        Self::new(None)
    }
}

pub trait Agent_Structure{
    fn new(
        id: Option<usize>,
        name: String,
        actions : Option<Vec<u8>>, 
        belief : Option<Vec<Fn>>,
        goal : Option<bool>,
        plan : Option<Vec<Fn>> ,
        utility :  Option<f64>) -> Result<Self, String> where Self: Sized;
    fn add_action(&mut self , action : u8) ->();
    fn add_belief(&mut self, beieve : Fn) ->();
    fn add_plan(&mut self, Plan : Vec<Fn>) ->();
    fn get_id(&self) -> ID;
    fn get_name(&self) -> &str;
    fn get_belief(&self) -> &Vec<Fn>;
    fn get_actions(&self) -> &Vec<u8>;
    fn get_goal(&self) -> &bool;
    fn get_plan(&self) -> &Vec<Fn>;
    fn add_goal(&mut self, Argument : bool) -> ();
    fn update_utitliy(&mut self, point : f64) -> ();
}

pub struct Agent{
    id : ID,
    name : String,
    pub belief : Vec<Fn> , 
    pub actions : Vec<u8>  ,
    pub goal : bool , 
    pub utilty : f64,
}



impl  Agent {
    pub fn new( id :  Option<usize> ,
                name : String ,
                actions : Option<Vec<u8>>,
                belief : Option<Vec<Fn>>,
                goal : Option<bool>, 
                // Plan :  Option<Vec<Fn>> ,
                utility : Option<f64>
            ) -> Result<Self, String>{
        let current = COUNTER.load(Ordering::SeqCst);
        let id: ID = match id {
            Some(id) =>{
                if id.to_string().len() >= 5 {return Err("Agent ID must not be More than 5 digit".to_string())}
                if id < current {return Err("Id Must be unique, curent id : ".to_string() + &id.to_string() )}
                ID::new(Some(id))
            }
            None => ID::default()};
            // COUNTER.store(Some(id), Ordering::SeqCst);

        Ok(
        Self{
            id : id,
            name ,
            actions : actions.unwrap_or(vec![]),
            belief : belief.unwrap_or(vec![]),
            goal : goal.unwrap_or(false),
            utilty : utility.unwrap_or(0.)
            // plan : Plan.unwrap_or(default)
            })
        }
    pub fn add_action(&mut self , predicate : u8){
        self.actions.push(predicate);
        }

//     pub fn add_belief(&mut self , predicate : Predicate){
//         self.belief.push(predicate);
//     }
    pub fn add_goal(&mut self, Argument : bool){
        self.goal = Argument;
        }
    pub fn update_utitliy(&mut self , point : f64){
        self.utilty += point;
    }
    pub fn get_id(&self) -> ID {
        self.id.clone()
        }
    pub fn get_name(&self) -> &str {
            &self.name
            }
    pub fn add_belief(&mut self, beieve : Fn) ->() {
        self.belief.push(beieve);
    }
    pub fn add_plan(&mut self, Plan : Vec<Fn>) ->() {
        // self.actions.extend(Plan);
        todo!()
    }
    pub fn get_belief(&self) -> &Vec<Fn> {
        &self.belief
        }
    pub fn get_actions(&self) -> &Vec<u8> {
        &self.actions
    }
    pub fn get_goal(&self) -> &bool {
        &self.goal
    }

    pub fn get_plan(&self) -> &Vec<Fn> {
        todo!()
    }

}

impl Display for Agent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "\nAgent {{ id: {}, \n
    name: {}, \n
    actions: {:?},\n
    goal  :  {:?} ,\n
    utilty : {},
    }}", 
    self.id, 
    self.name,  
    self.actions, 
    // self.belief,     // belief : {#?},\n
    self.goal,
    self.utilty)}

    
}