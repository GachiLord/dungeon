use reqwest::Client;
use serde::Serialize;
use serde_tuple::*;

use crate::{entities::task::Task, AI_HOST};

// User example:  [2, 7, ["JavaScript", "React", "CSS", "HTML"]]'
//
// Tasks example:
// [
//  [2, 7, ["Linux", "Docker"]],
//  [2, 9, ["Swift", "iOS Development"]],
//  [1.5, 7, ["Kotlin", "Android Development"]]
// ]

#[derive(Deserialize_tuple, Serialize_tuple, Debug, PartialEq)]
struct Params {
    complexity: f32,
    time: f32,
    tags: Vec<Box<str>>,
}

#[derive(Serialize)]
struct TaskRequest {
    worker: Params,
    tasks: Vec<Params>,
}

pub async fn get_recommended(
    http_client: Client,
    user_complexity: f32,
    user_time: f32,
    user_tags: Vec<Box<str>>,
    tasks: &Vec<Task>,
) -> Result<Vec<usize>, reqwest::Error> {
    let t_params = tasks
        .iter()
        .map(|t| {
            let c: i16 = t.complexity.into();
            Params {
                complexity: c as f32,
                time: t.expected_time,
                tags: t.tags.clone(),
            }
        })
        .collect();
    let u_params = {
        Params {
            complexity: user_complexity,
            time: user_time,
            tags: user_tags,
        }
    };

    let body = TaskRequest {
        worker: u_params,
        tasks: t_params,
    };
    let response: Vec<Params> = http_client
        .post(*AI_HOST)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    let tasks = body.tasks;
    let mut indexes = vec![];

    // check first 5 recommendations
    // TODO: use hashset to avoid nested loop
    for (i, task) in tasks.iter().enumerate() {
        for j in 0..5 {
            if Some(task) == response.get(j) {
                indexes.push(i);
            }
        }
    }

    Ok(indexes)
}
