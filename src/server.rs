use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};

use serde::{Deserialize, de::IntoDeserializer};

use crate::db::{data_structures::User, get_db};

pub async fn create_server() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/helloworld", get(helloworld_handler))
        .route("/create_user", post(create_user))
        .route("/update_user",post(update_user))
        .route("/get_user_data",post(get_user_data));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running");

    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Html<String> {
    return Html::from(String::from("Hello world! html"));
}

async fn helloworld_handler() -> Html<String> {
    return Html::from(String::from("This is a generic response"));
}

// Needed handlers. All take JSON input (json API)

// Case 1 - Create a user : Inputs: Full user structure, without ratings but with pics (optional). Outputs: OK HTTP / ERROR HTTP

async fn create_user(body: String) -> impl IntoResponse {
    let pl: Result<User, _> = serde_json::from_str(&body);
    let user: User;
    match pl {
        Ok(r) => {
            user = r;
        }
        Err(_err) => {
            return Err((StatusCode::BAD_REQUEST, "Invalid JSON data").into_response());
        }
    }
    println!("{}", user.username);
    let r = get_db().await.create_user(&user).await;
    match r {
        Ok(r) => {
            return Ok(format!("created user {}", r).into_response());
        }
        Err(err) => {
            println!("{}",err);
            return Err((StatusCode::BAD_REQUEST, "Invalid username").into_response());
        }
    }
}

// Case 2 - Update user attributes: Inputs: Full user structure. Pics and ratings wont be upated. Outputs OK HTTP / ERROR HTTP

async fn update_user(body: String) -> impl IntoResponse {
    let pl: Result<User, _> = serde_json::from_str(&body);
    let user: User;
    match pl {
        Ok(r) => {
            user = r;
        }
        Err(_err) => {
            return Err((StatusCode::BAD_REQUEST, "Invalid JSON data").into_response());
        }
    }

    let r = get_db().await.update_user(&user).await;
    match r {
        Ok(r) => {
            return Ok(format!("updated user {}", r).into_response());
        }
        Err(err) => {
            println!("{}",err);
            return Err((StatusCode::BAD_REQUEST, "Invalid username").into_response());
        }
    }
}

// Case 3 - Get my data: Inputs: user id. Outputs: Full user structure without ratings but with pics

pub async fn get_user_data(body: String) -> impl IntoResponse {
    let pl: Result<User, _> = serde_json::from_str(&body);
    let user: User;
    match pl {
        Ok(r) => {
            user = r;
        }
        Err(_err) => {
            return Err((StatusCode::BAD_REQUEST, "Invalid JSON data").into_response());
        }
    }
    
    let r = get_db().await.get_user_data(&user).await;
    match r {
        Ok(r) => {
            return Ok(serde_json::to_string(&r).unwrap().into_response()); // todo this unwrap is unsafe
        }
        Err(err) => {
            println!("{}",err);
            return Err((StatusCode::BAD_REQUEST, "Invalid username").into_response());
        }
    }
}

// Case 5 - Get ratings performed on me. Inputs: user id. Outputs: List of all ratings

// Case 6 - Get ratings performed by me. Inputs: user id. Outputs: List of all ratings

// Case 7 - Get next rating profile: Inputs: None. Outputs: Full user structure without ratings but with pics

// Case 8 - Remove pic: Inputs: user id, pic url. Output: None (Ok/error)

// Case 9 - Add pic: Inputs: user id ... Somehow upload pic? Idk how to do this

// ------------------------------------------------------------------------------------------------------------------------------------------

// Picture handlers (probably a separate class)

// Case 1: fetch pic: Inputs: pic URL. Outputs: pic

// Case 2: delete pic: Will delete a pic from disk. Inputs: pic URL. Outputs: None

// Case 3: create pic: Will create a pic on disk and generate a URL for it
