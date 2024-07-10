use derive_more::Display;
use std::usize;
use std::sync::atomic::{AtomicUsize, Ordering};


// Atomic counter for generating unique IDs
static COUNTER: AtomicUsize = AtomicUsize::new(1);


#[derive(Clone, Debug, Display,Default)]
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


pub trait AgentTrait{
    fn new( id :  Option<usize> ,
        name : String ,
        actions : Option<Vec<u8>>,
        state_action : Option<Vec<Vec<f64>>>,
        utility :  Option<f64>
    ) -> Result<Self, String> where Self: Sized;
    
    fn add_action(&mut self , action : u8) ->();

}

#[derive(Display)]
#[display(fmt = "Agent(Id:{id}, Name: {name}, Actions: {actions:?},  Utility: {state_action:?})")]
pub struct Agent{
    #[display("ID({_0})")]
    id : ID,
    #[display("name {_0}")]
    name : String,
    #[display("Action {_0}")]
    pub actions : Vec<u8>  ,
    #[display("state_action {_0}")]
    pub state_action :  Vec<Vec<f64>>,
    #[display("utility {_0}")]
    utility : Option<f64>

}




impl  AgentTrait for Agent {
    fn new( id :  Option<usize> ,
                name : String ,
                actions : Option<Vec<u8>>,
                state_action : Option<Vec<Vec<f64>>>,
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

        Ok(
        Self{
            id : id,
            name ,
            actions : actions.unwrap_or(vec![]),
            state_action : state_action.unwrap_or(vec![vec![]]),
            utility : Some(utility.unwrap_or(0.0))
            })
        }
    fn add_action(&mut self , predicate : u8){
        self.actions.push(predicate);
        }

}


