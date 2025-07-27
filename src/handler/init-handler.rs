
use crate::handler::Handler;

pub struct InitParams { pub verbose: bool }

pub struct InitHandler {}

impl Handler<InitParams, ()> for InitHandler {
    fn execute(&self, params: InitParams) -> Result<(), Box<dyn std::error::Error>> {
        
        Ok(())
    }
}
