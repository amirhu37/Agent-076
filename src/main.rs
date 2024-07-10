
use rand::prelude::*;

use agent_046::{cls, datatype, mean};
use agent_046::agent::{Agent, AgentTrait};
use agent_046::action::*;


#[allow(dead_code)]
#[derive(Debug)]
enum AgentActions{
    Up = 0,
    Down = 1
}
use crate::AgentActions::*;

impl AgentActions {

    fn up() -> Self{
        println!("Agent is moving up");
        Up
    }
    fn down() -> Self {
        println!("Agent is moving down");
        Down
        }
            
}


impl ActionTemplate for AgentActions {
    fn execute(command : String)-> Result<AgentActions, &'static str>{
        match command.as_str(){
            "up" => Ok(AgentActions::up()) ,
            "down" => Ok(AgentActions::down()),
            _ => Err("invaid action")
            }

    }
}


use rand::seq::SliceRandom;

fn main() {
    cls();
    let state_action: Vec<Vec<f64>> = vec![vec![0., 2. ,3. ],
                    vec![0.0, 2.0,3.0 ],
                    vec![0.0, 2.0 ,3.0 ]];
    // let exe =  AgentActions::execute("up".to_string());
    let mut alice: Agent = Agent::new( None, 
                                    "Alice".to_string() , 
                                    None, 
                                    Some(state_action) , None).unwrap();
    alice.add_action(0);
    alice.add_action(1);
        println!("{}", datatype(&AgentActions::Down));
    println!("{}  \n", &alice);

    for _ in 1..=1200{
        let mut rng: ThreadRng = rand::thread_rng();
        let y: f64 = rng.gen();
        let x: &u8  = alice.actions.choose(&mut rng).unwrap_or(&0) ;   
        match (y, x) {
            (y, &0) if y > 0.5 => alice.state_action[0][2] += 0.5,
            (y, &0) if y < 0.5 => alice.state_action[0][1] *= 0.0,
            (y, &1) if y > 0.5 => alice.state_action[1][1] = 0.0,
            (y, &1) if y < 0.5 => alice.state_action[1][0] -= 0.4,
            (y, &1) if y >= 0.5 => alice.state_action[0][2] -= 0.5,
            _ => (),
        }
    }
    // let mad_hatter: Agent = Agent::new( Some(2), "Mad Hatter".to_string(), None,  None, None).unwrap();
    println!("{}  \n", &alice);
    println!("{} \n" , mean(&alice.state_action))
    


 }
