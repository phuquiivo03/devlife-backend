use axum::{http::Result, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};


#[derive(Debug, Serialize, Clone)]
pub struct Hacker {
    id: String,
    title: String,
    stake: u32,
    max_stake: u32,
    hacker_point: u32,
    level: u32
}
#[derive(Debug, Deserialize, Clone)]
pub struct HackerForCreate {
    id: String,
    title: String,
    max_stake: u32,
}

impl Hacker {
    pub fn new(
        hacker_fc: HackerForCreate
    ) -> Hacker {
        Hacker {
            id: hacker_fc.id,
            title: hacker_fc.title,
            stake: 0,
            max_stake: hacker_fc.max_stake,
            hacker_point: 0,
            level: 1
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn update_stake(&self, stake: u32) -> Hacker {
        Hacker {
            id: self.id.clone(),
            title: self.title.clone(),
            stake: stake,
            max_stake: self.max_stake,
            hacker_point: self.hacker_point,
            level: self.level
        }
    }

    pub fn edit_title(&self, title: String) -> Hacker {
        Hacker {
            id: self.id.clone(),
            title: title,
            stake: self.stake,
            max_stake: self.max_stake,
            hacker_point: self.hacker_point,
            level: self.level
        }
    }

    pub fn level_up(&self) -> Hacker {
        Hacker {
            id: self.id.clone(),
            title: self.title.clone(),
            stake: 0,
            max_stake: self.max_stake, // how to get new max stake?
            hacker_point: self.hacker_point,
            level: self.level + 1
        }
    }
}


impl HackerForCreate {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
