use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Serialize)]
struct Choice {
    message: ChatMessage,
    index: usize,
}

#[post("/v1/chat/completions")]
async fn chat_completions(req: web::Json<ChatCompletionRequest>) -> impl Responder {
    let last_message = if let Some(message) = req.messages.last() {
        message
    } else {
        &ChatMessage {
            role: "user".to_string(),
            content: "Hello!".to_string(),
        }
    };

    let simulated_response = "Simulated response to: ".to_string() + &last_message.content;

    let response = ChatCompletionResponse {
        id: "chat-completion-simulation".into(),
        object: "chat.completion".into(),
        model: req.model.clone(),
        choices: vec![Choice {
            message: ChatMessage {
                role: "assistant".into(),
                content: simulated_response,
            },
            index: 0,
        }],
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(chat_completions))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}