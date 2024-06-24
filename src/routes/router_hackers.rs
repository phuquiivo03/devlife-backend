use std::collections::HashMap;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};
use crate::{controllers::hacker_controller::{self, HackerController}, models::hacker::{Hacker, HackerForCreate}};
use crate::error::{Result, Error};

pub fn routers(hc: HackerController) -> Router {
    Router::new()
        .route("/hackers", get(get_hackers))
        .route("/hackers", post(new_hacker))
        .with_state(hc)
}



pub async fn new_hacker(
    State(hacker_controller): State<HackerController>,
    Json(hacker_fc): Json<HackerForCreate>
) -> impl IntoResponse {
    let hackers = hacker_controller.get_hackers().await;
    if hackers.contains_key(&hacker_fc.get_id()) {
        return Err(Error::FailedToCreateHacker);
    }
    let hacker = Hacker::new(hacker_fc.into());
    hacker_controller.add_hacker(hacker.clone()).await?;
    Ok(Json(hacker))
}


pub async fn get_hackers(
    State(hacker_controller): State<HackerController>

) -> Result<Json<HashMap<String, Hacker>>> {
    Ok(Json(hacker_controller.get_hackers().await))
}