


use LogicLanguague::agent::Agent;


// use std::collections::HashSet;
// use std::fmt;
fn main() {
    // let mut agent = Agent::new("Alice".to_string());
    // agent.add_belief("it is raining".to_string());
    // agent.add_belief("I have an umbrella".to_string());
    // agent.add_goal("stay dry".to_string());
    // agent.add_goal("go outside".to_string());
    // agent.add_action("take umbrella".to_string(), "I have an umbrella".to_string());
    
    // let predicate1 = Predicate::new("agent".to_string(), 
    //                         true,
    //                          "description".to_string());
    // let var = VecPredicate::new(vec!["Are".to_string()]);
    // let action1 = Action::new();
    // println!("{}", predicate1);
    let alice: Agent = Agent::new( None, "Alice".to_string());
    let mad_hatter: Agent = Agent::new( Some(1), "Mad Hatter".to_string());

    println!("{}", alice);
    println!("{}", mad_hatter);
 }
