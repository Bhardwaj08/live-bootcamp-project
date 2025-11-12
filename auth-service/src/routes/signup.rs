use axum::{extract::State, http::{request, StatusCode}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{app_state::AppState, domain::user::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> impl IntoResponse {

    let email = request.email.clone();
    let password = request.password.clone();
    let user = User::new(email, password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully".to_string(),
    });

    (StatusCode::CREATED, response);
    // StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename="requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}