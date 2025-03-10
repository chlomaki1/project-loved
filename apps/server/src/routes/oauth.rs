use actix_web::{get, web, Responder};
use athena::{entities::users, prelude::users::FullUser};
use redis::Commands;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};

use crate::{errors::LovedError, service::{self, Response}, state::LovedState};


#[derive(Deserialize)]
struct StartTokenRequest {
    #[serde(alias = "type")]
    token_type: String,
}

#[derive(Serialize)]
struct StartTokenResponse {
    state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct TokenState {
    token_type: String,
    token: String,
    scopes: Vec<String>
}

#[get("/token")]
pub async fn start_token(
    state: web::Data<LovedState>,
    query: web::Query<StartTokenRequest>
) -> impl Responder {
    if query.token_type == "login" {
        let token_state = TokenState {
            token_type: query.token_type.clone(),
            token: service::generate_token(),
            scopes: vec!["identify".into(), "public".into()]
        };
        
        let encoded_state = serde_json::to_string(&token_state).unwrap();
        let _ = state.cache(|con| {
            redis::pipe() 
                .sadd("loved:login-tokens", encoded_state.clone())
                .ignore()
                .query::<()>(con)?;
            Ok(())
        }).await;

        Ok(Response {
            status: 200,
            message: None,
            data: Some(StartTokenResponse {
                state: encoded_state.clone(),
                url: Some(format!(
                    "{}/oauth/authorize?client_id={}&redirect_uri={}/login&response_type=code&scope=identify public&state={}",
                    state.env.get::<String>("OSU_URL").unwrap(),
                    state.env.get::<String>("OSU_CLIENT_ID").unwrap(),
                    state.env.get::<String>("OSU_REDIRECT_URI").unwrap(),
                    encoded_state.clone(),
                ))
            })
        })
    } else {
        Err(LovedError::InvalidTokenAuthType)
    }
}

#[derive(Deserialize)]
struct TokenCallbackRequest {
    code: String,
    state: String
}

#[get("/callback/login")]
pub async fn login_token_callback(
    state: web::Data<LovedState>,
    query: web::Query<TokenCallbackRequest>
) -> impl Responder {
    let token_state = serde_json::from_str(query.state.clone().as_str());

    if token_state.is_err() {
        return Err(LovedError::InvalidTokenState);
    }

    let token_state: TokenState = token_state.unwrap();

    if token_state.token_type == "login" {
        let existing_state = state.cache(|con| {
            let exists: i32 = con.sismember("loved:login-tokens", query.state.clone())?;
            
            if exists == 0 {
                return Ok(false);
            }

            redis::pipe()
                .srem("loved:login-tokens", query.state.clone())
                .ignore()
                .query::<()>(con)?;
        
            Ok(exists == 1)
        }).await;

        if existing_state.is_err() {
            return Err(LovedError::InvalidTokenState);
        }

        if let Ok(false) = existing_state {
            Err(LovedError::InvalidTokenState)
        } else {
            let user_client = state.get_osu_client(query.code.clone(), "/login").await?;
            let user = user_client.own_data().await?;
            let mut display_user = None;

            // TODO: Move this to a seperate function that does this
            if let Ok(existing) = FullUser::fetch(user.user_id.try_into().unwrap_or(0), &state.db_pool).await {
                let mut existing = existing.base.into_active_model();
                
                if existing.username.as_ref() != &user.username.to_string() {
                    // TODO: Store previous usernames
                    existing.set(users::Column::Username, user.username.to_string().into());
                }
            
                if existing.is_changed() {
                    display_user = Some(FullUser::update(existing, &state.db_pool).await?.into_display());
                }
            } else {
                display_user = Some(FullUser::create(users::ActiveModel {
                    id: sea_orm::ActiveValue::Set(user.user_id.try_into().unwrap()),
                    username: sea_orm::ActiveValue::Set(user.username.to_string()),
                    country: sea_orm::ActiveValue::Set(Some(user.country_code.to_string())),
                    restricted: sea_orm::ActiveValue::Set(user.is_restricted.unwrap_or(false)),
                    api_fetched_at: sea_orm::ActiveValue::Set(chrono::Utc::now().naive_utc()),
                    tokens: sea_orm::ActiveValue::Set(serde_json::json!({})) // TODO: Securely store tokens
                }, &state.db_pool).await?.into_display());
            }

            if let Some(mut display_user) = display_user {
                display_user.obtain_roles(&state.db_pool).await?;

                Ok(Response {
                    status: 200,
                    message: None,
                    data: Some(display_user)
                })
            } else {
                // This shouldn't be possible.
                Err(LovedError::InternalError)
            }
        }
    } else {
        Err(LovedError::InvalidTokenAuthType)
    }
}