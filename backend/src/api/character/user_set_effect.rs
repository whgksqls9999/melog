use crate::api::character::request::request_parser;
use crate::api::request::API;

use super::character::UserOcid;

use axum::{Extension, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetEffectInfoFull {
    set_count: i8,
    set_option: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetEffectInfo {
    set_name: String,
    total_set_count: i8,
    set_option_full: Vec<SetEffectInfoFull>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetEffect {
    set_effect: Vec<SetEffectInfo>,
}

pub async fn get_user_set_effect(
    Extension(api_key): Extension<Arc<API>>,
    Json(user_ocid): Json<UserOcid>,
) -> Result<Json<SetEffect>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), "set-effect", &user_ocid.ocid).await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_effect: SetEffect = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        let filtered_data = SetEffect {
            set_effect: user_effect
                .set_effect
                .into_iter()
                .filter_map(|set_info| {
                    let matched_options: Vec<SetEffectInfoFull> = set_info
                        .set_option_full
                        .into_iter()
                        .filter(|option| option.set_count <= set_info.total_set_count)
                        .collect();

                    if matched_options.is_empty() {
                        None
                    } else {
                        Some(SetEffectInfo {
                            set_name: set_info.set_name,
                            total_set_count: set_info.total_set_count,
                            set_option_full: matched_options,
                        })
                    }
                })
                .collect(),
        };

        Ok(Json(filtered_data))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}
