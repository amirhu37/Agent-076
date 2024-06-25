
use LogicLanguague::agent::Agent;

#[allow(dead_code)]
trait ActionTemplate {}

#[allow(dead_code)]

#[derive(Debug)]
enum AgentActions{
    Up,
    Down
}

impl ActionTemplate for AgentActions {}


fn cls(){
    use std::process::Command;
    Command::new("cmd")
    .args(&[ "cls"])
    .output()
    .expect("failed to execute process");
}

fn main() {
    cls();


    let mut alice: Agent = Agent::new( None, "Alice".to_string() , None, None, None, None).unwrap();
    let mad_hatter: Agent = Agent::new( Some(2), "Mad Hatter".to_string(), None, None, None, None).unwrap();
    println!("{}", &alice.utilty);
    alice.utilty = 0.1 ;
    println!("{}", &alice.utilty);
    alice.update_utitliy(3.);
    println!("{}", alice.utilty);
    alice.actions = vec![2 , 1] ;

    // let agent: AgentActions = match alice.actions[0] {
    //     1 => AgentActions::Up ,
    //     2 => AgentActions::Down ,
    // };
    println!("{:?}", alice.actions); 
 }
