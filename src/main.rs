
use agent_046::env::{Env, MyEnv};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use rand::prelude::*;
// use derive_more::Display as cDisplay;

use agent_046::{cls, learn};
use agent_046::agent::{ Agent, MyAgent};

use ndarray::{array, Array1, Array2};



fn main() {
    cls();

    let state_action: Array2<f64> = Array2::random((120,30), Uniform::new(0., 1.));
    let acts: Array1<i32>  = array![0, 1 , 2] ;

    let mut alice: MyAgent = MyAgent::new( None, 
                                    "Alice".to_string() , 
                                    Some(acts), 
                                    Some(state_action) , Some(0.)).unwrap();
    // let mad_atter: Agent = Agent::new( Some(2), "Mad Hatter".to_string(), None,  None, None).unwrap();

    let mut env = MyEnv::new(Some(1),
                         "name".to_string(), alice.actions.clone(),
                          Some(vec![1. ,2.])).unwrap();

    // // To store the final state and reward from each episode
    let mut results: Vec<(f64, f64)> = Vec::new();
    for _ in 1..=1200{
        let action_size = alice.q_tabel.shape()[1] ;
        let state_size = alice.q_tabel.shape()[0] ;

        // Create a random number generator
        let mut rng = rand::thread_rng();

        let mut done = false;
        while !done {
            let action = &rng.gen_range(0..action_size);
            // let action = action_size[random_number];

            let state = rng.gen_range(0..state_size);
            let (state, reward, _done, _) = env.step(*action as i32 , state as f64);
            // let reutrn = alice.policy(state) ;       
            done = _done;
            alice.utility = Some(alice.utility.unwrap() + reward);
            alice.q_tabel = learn(0.1, 
                0.5, reward, 
                *action, 
                state as usize, alice.q_tabel);

            results.push((state, reward));
    
           }
    //      println!("{:?} {} {:?} {}", results[results.len() -1].0, results[results.len() -1].1, &alice.utility.unwrap(), done);
        };

        println!("{}  \n", &alice.q_tabel);
        println!("{:?} \n" , &alice.q_tabel.mean().unwrap());
    //     println!("state {} reward {} utility {}", 
    //     results[results.len() -1].0, results[results.len() -1].1, &alice.utility.unwrap());
 }
