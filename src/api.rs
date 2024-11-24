use warp::Filter;
use std::sync::Arc;
use crate::{executor, storage, rejections};
use serde_json::Value;

pub fn server(
    storage: Arc<storage::Storage>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(with_storage(storage.clone()))
        .and_then(register_function);

    let invoke = warp::post()
        .and(warp::path("invoke"))
        .and(warp::body::json())
        .and(with_storage(storage.clone()))
        .and_then(invoke_function);

    register.or(invoke)
}

fn with_storage(
    storage: Arc<storage::Storage>,
) -> impl Filter<Extract = (Arc<storage::Storage>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

async fn register_function(
    body: Value,
    storage: Arc<storage::Storage>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let function_name = body["name"]
        .as_str()
        .ok_or_else(|| warp::reject::custom(rejections::InvalidParameter {
            message: "Missing or invalid 'name' parameter".to_string(),
        }))?;

    let code = body["code"]
        .as_str()
        .ok_or_else(|| warp::reject::custom(rejections::InvalidParameter {
            message: "Missing or invalid 'code' parameter".to_string(),
        }))?;

    storage
        .save_function(function_name.to_string(), code.to_string())
        .map_err(|_| warp::reject::custom(rejections::InvalidParameter {
            message: "Failed to save function".to_string(),
        }))?;

    Ok(warp::reply::json(&format!("Function {} registered!", function_name)))
}

async fn invoke_function(
    body: Value,
    storage: Arc<storage::Storage>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let function_name = body["name"]
        .as_str()
        .ok_or_else(|| warp::reject::custom(rejections::InvalidParameter {
            message: "Missing or invalid 'name' parameter".to_string(),
        }))?;

    let input = body["input"]
        .as_array()
        .ok_or_else(|| warp::reject::custom(rejections::InvalidParameter {
            message: "Missing or invalid 'input' parameter".to_string(),
        }))?;

    let code = storage.load_function(function_name).map_err(|_| {
        warp::reject::custom(rejections::NotFound {
            message: format!("Function '{}' not found", function_name),
        })
    })?;

    let result = executor::execute(&code, function_name, input).map_err(|_| {
        warp::reject::custom(rejections::InvalidParameter {
            message: "Failed to execute function".to_string(),
        })
    })?;

    Ok(warp::reply::json(&result))
}
