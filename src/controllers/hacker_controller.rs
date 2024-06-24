use std::sync::{Arc, Mutex};
use serde::Serialize;
use serde_json;

use crate::error::{Error, Result};
use crate::models::hacker::{self, Hacker};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HackerController {
    hackers: Arc<Mutex<HashMap<String, hacker::Hacker>>>,
}


impl HackerController {
    pub fn new() -> HackerController {
        HackerController {
            hackers: Arc::new(Mutex::new(HashMap::new())),
        }
    }


    pub async fn add_hacker(&self, hacker: hacker::Hacker) -> Result<()> {
        let mut hackers = self.hackers.lock().unwrap();
        hackers.insert(hacker.get_id(), hacker);
        Ok(())
    }


    pub async fn get_hackers(&self) -> HashMap<String, hacker::Hacker> {
        let hackers = self.hackers.lock().unwrap();
        hackers.clone()
    }

}



