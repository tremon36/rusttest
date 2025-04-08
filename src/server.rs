use axum::{Router, response::Html, routing::get};

pub async fn create_server() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/helloworld", get(helloworld_handler));

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

// Case 2 - Update user attributes: Inputs: Full user structure. Pics and ratings wont be upated. Outputs OK HTTP / ERROR HTTP

// Case 3 - Get my data: Inputs: user id. Outputs: Full user structure without ratings but with pics 

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


