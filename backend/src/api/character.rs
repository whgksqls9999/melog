use crate::api::request::{API, request_parser};

use axum::Extension;
use axum::{
    http::{HeaderMap, StatusCode},
    response::Json,
};
use chrono::{Duration, Utc};
use chrono_tz::Asia::Seoul;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserOcid {
    ocid: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    nick_name: String,
}

pub async fn get_ocid(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
    Json(character): Json<Character>,
) -> Result<Json<UserOcid>, (StatusCode, &'static str)> {
    let uuid = header
        .get("uuid")
        .and_then(|value| value.to_str().ok())
        .ok_or((StatusCode::BAD_REQUEST, "Missing or invalid uuid header"))?;

    let client = Client::new();

    // 요청할 API의 URL
    let url = format!(
        "https://open.api.nexon.com/maplestory/v1/id?character_name={}",
        character.nick_name
    );
    // 요청 헤더 정의
    let mut headers = header::HeaderMap::new();
    headers.insert("x-nxopen-api-key", api_key.key.parse().unwrap());

    // POST 요청 보내기
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .expect("Failed to send request");

    // 응답 결과 확인
    if response.status().is_success() {
        let userocid: UserOcid = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        // 전역 변수 업데이트
        api_key.set_ocid_uuid(uuid.to_string(), userocid.ocid.clone());

        Ok(Json(userocid))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDefaultData {
    character_name: String,
    world_name: String,
    character_gender: String,
    character_class: String,
    character_class_level: String,
    character_level: i16,
    character_exp: i64,
    character_exp_rate: String,
    character_guild_name: String,
    character_image: String,
    character_date_create: String,
}

pub async fn get_user_default_info(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<UserDefaultData>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "basic").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_data: UserDefaultData = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_data))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stat {
    pub stat_name: String,
    pub stat_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserStatData {
    pub final_stat: Vec<Stat>,
}

pub async fn get_user_stat_info(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<UserStatData>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "stat").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_stat_data: UserStatData = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_stat_data))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HyperStat {
    pub stat_type: String,
    pub stat_point: Option<u32>, // null을 허용하기 위해 Option 사용
    pub stat_level: u32,
    pub stat_increase: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserHyperStatData {
    pub hyper_stat_preset_1: Vec<HyperStat>,
    pub hyper_stat_preset_1_remain_point: i32,
    pub hyper_stat_preset_2: Vec<HyperStat>,
    pub hyper_stat_preset_2_remain_point: i32,
    pub hyper_stat_preset_3: Vec<HyperStat>,
    pub hyper_stat_preset_3_remain_point: i32,
}

pub async fn get_user_hyper_stat_info(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<UserHyperStatData>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "hyper-stat").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_hyper_stat_data: UserHyperStatData = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        let filtered_data = UserHyperStatData {
            hyper_stat_preset_1: user_hyper_stat_data
                .hyper_stat_preset_1
                .into_iter()
                .filter(|stat| stat.stat_point.is_some() && stat.stat_increase.is_some())
                .collect(),
            hyper_stat_preset_1_remain_point: user_hyper_stat_data.hyper_stat_preset_1_remain_point,

            hyper_stat_preset_2: user_hyper_stat_data
                .hyper_stat_preset_2
                .into_iter()
                .filter(|stat| stat.stat_point.is_some() && stat.stat_increase.is_some())
                .collect(),
            hyper_stat_preset_2_remain_point: user_hyper_stat_data.hyper_stat_preset_2_remain_point,

            hyper_stat_preset_3: user_hyper_stat_data
                .hyper_stat_preset_3
                .into_iter()
                .filter(|stat| stat.stat_point.is_some() && stat.stat_increase.is_some())
                .collect(),
            hyper_stat_preset_3_remain_point: user_hyper_stat_data.hyper_stat_preset_3_remain_point,
        };

        Ok(Json(filtered_data))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Propensity {
    pub charisma_level: i8,
    pub sensibility_level: i8,
    pub insight_level: i8,
    pub willingness_level: i8,
    pub handicraft_level: i8,
    pub charm_level: i8,
}

pub async fn get_user_propensity(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<Propensity>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "propensity").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_propensity: Propensity = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_propensity))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AbilityInfo {
    pub ability_no: String,
    pub ability_grade: String,
    pub ability_value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ability {
    pub ability_grade: String,
    pub ability_info: Vec<AbilityInfo>,
}

pub async fn get_user_ability(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<Ability>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "ability").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_ability: Ability = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_ability))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

// TODO : 사용자 착용 아이템 정보
#[derive(Deserialize, Serialize, Debug)]
pub struct ItemEquipmentInfoOption {
    pub str: String,
    pub dex: String,
    pub int: String,
    pub luk: String,
    pub max_hp: String,
    pub max_mp: String,
    pub attack_power: String,
    pub magic_power: String,
    pub armor: String,
    pub speed: String,
    pub jump: String,
    pub boss_damage: String,
    pub ignore_monster_armor: String,
    pub all_stat: String,
    #[serde(default)]
    pub damage: String,
    #[serde(default)]
    pub equipment_level_decrease: i8,
    pub max_hp_rate: String,
    pub max_mp_rate: String,
    #[serde(default)]
    pub base_equipment_level: i16,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ItemEquipmentInfoExceptionalOption {
    pub str: String,
    pub dex: String,
    pub int: String,
    pub luk: String,
    pub max_hp: String,
    pub max_mp: String,
    pub attack_power: String,
    pub magic_power: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub exceptional_upgrade: i16,
    #[serde(default)]
    pub armor: String,
    #[serde(default)]
    pub speed: String,
    #[serde(default)]
    pub jump: String,
    #[serde(default)]
    pub damage: String,
    #[serde(default)]
    pub all_stat: String,
    #[serde(default)]
    pub equipment_level_decrease: i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ItemEquipmentStatOption {
    pub str: String,
    pub dex: String,
    pub int: String,
    pub luk: String,
    pub max_hp: String,
    pub max_mp: String,
    pub attack_power: String,
    pub magic_power: String,
    pub armor: String,
    pub speed: String,
    pub jump: String,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ItemEquipmentInfo {
    pub item_equipment_part: String,
    pub item_equipment_slot: String,
    pub item_name: String,
    pub item_icon: String,
    pub item_shape_name: String,
    pub item_shape_icon: String,
    pub item_total_option: ItemEquipmentInfoOption,
    pub item_base_option: ItemEquipmentInfoOption,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub potential_option_grade: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub additional_potential_option_grade: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub potential_option_1: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub potential_option_2: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub potential_option_3: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub additional_potential_option_1: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub additional_potential_option_2: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub additional_potential_option_3: String, // null 가능
    pub item_exceptional_option: ItemEquipmentInfoExceptionalOption,
    pub item_add_option: ItemEquipmentInfoExceptionalOption,
    pub scroll_upgrade: String,
    pub cuttable_count: String,
    pub golden_hammer_flag: String,
    pub scroll_resilience_count: String,
    pub scroll_upgradeable_count: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub soul_name: String, // null 가능
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub soul_option: String, // null 가능
    pub starforce: String,
    pub item_etc_option: ItemEquipmentStatOption,
    pub item_starforce_option: ItemEquipmentStatOption,
    pub special_ring_level: i8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ItemEquipment {
    pub item_equipment: Vec<ItemEquipmentInfo>,
}

pub async fn get_user_item_equipment(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<ItemEquipment>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "item-equipment").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_item_equipment: ItemEquipment = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_item_equipment))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

// TODO : 캐시 사용자 착용 아이템 정보

#[derive(Deserialize, Serialize, Debug)]
pub struct SymbolInfo {
    pub symbol_name: String,
    pub symbol_icon: String,
    pub symbol_force: String,
    pub symbol_level: i8,
    pub symbol_str: String,
    pub symbol_dex: String,
    pub symbol_int: String,
    pub symbol_luk: String,
    pub symbol_hp: String,
    pub symbol_drop_rate: String,
    pub symbol_meso_rate: String,
    pub symbol_exp_rate: String,
    pub symbol_growth_count: i32,
    pub symbol_require_growth_count: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Symbol {
    pub symbol: Vec<SymbolInfo>,
}

pub async fn get_user_symbol_equipment(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<Symbol>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "symbol-equipment").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_symbol: Symbol = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_symbol))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetEffectInfoFull {
    pub set_count: i8,
    pub set_option: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetEffectInfo {
    pub set_name: String,
    pub total_set_count: i8,
    pub set_option_full: Vec<SetEffectInfoFull>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetEffect {
    pub set_effect: Vec<SetEffectInfo>,
}

pub async fn get_user_set_effect(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<SetEffect>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "set-effect").await;

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

// TODO : 장착 헤어, 성형, 피부 정보 조회
// TODO : 장착 안드로이드 조회
// TODO : 장착 펫 정보 조회

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct SkillInfo {
    pub skill_name: String,
    pub skill_description: String,
    pub skill_level: i8,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub skill_effect: String,
    pub skill_icon: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub skill_effect_next: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CharacterSkill {
    pub character_skill: Vec<SkillInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CharacterSkilLevel {
    pub level: i8,
}

pub async fn get_user_characeter_skill(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
    Json(character_skil_level): Json<CharacterSkilLevel>,
) -> Result<Json<CharacterSkill>, (StatusCode, &'static str)> {
    let uuid = header
        .get("uuid")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    // 요청 헤더 정의
    let mut headers = header::HeaderMap::new();
    headers.insert("x-nxopen-api-key", api_key.key.parse().unwrap());

    let now_time = (Utc::now() - Duration::days(1))
        .with_timezone(&Seoul)
        .format("%Y-%m-%d");

    let url = format!(
        "https://open.api.nexon.com/maplestory/v1/character/skill?ocid={}&date={}&character_skill_grade={}",
        api_key.get_ocid_uuid(uuid).unwrap_or_default(),
        now_time,
        character_skil_level.level
    );

    // POST 요청 보내기
    let response = Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await
        .expect("Failed to send request");

    // 응답 결과 확인
    if response.status().is_success() {
        let user_character_skill: CharacterSkill = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_character_skill))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CharacterLinkSkill {
    pub character_link_skill: Vec<SkillInfo>,
}

pub async fn get_user_characeter_link_skill(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<CharacterLinkSkill>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "link-skill").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_character_link_skill: CharacterLinkSkill = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_character_link_skill))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct VMatrixInfo {
    pub slot_id: String,
    pub slot_level: i8,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub v_core_name: String,
    pub v_core_level: i8,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub v_core_skill_1: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub v_core_skill_2: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub v_core_skill_3: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub v_core_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VMatrix {
    pub character_v_core_equipment: Vec<VMatrixInfo>,
    pub character_v_matrix_remain_slot_upgrade_point: i8,
}

pub async fn get_user_v_matrix(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<VMatrix>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "vmatrix").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_v_matrix: VMatrix = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_v_matrix))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HexaSkillInfo {
    pub hexa_skill_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HexaMatrixInfo {
    pub hexa_core_name: String,
    pub hexa_core_level: i8,
    pub hexa_core_type: String,
    pub linked_skill: Vec<HexaSkillInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HexaMatrix {
    pub character_hexa_core_equipment: Vec<HexaMatrixInfo>,
}

pub async fn get_user_hexa_matrix(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<HexaMatrix>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "hexamatrix").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_hexa_matrix: HexaMatrix = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_hexa_matrix))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}

// TODO : Hexa 매트릭스 설정 정보 조회
#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct Dojang {
    pub dojang_best_floor: i8,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub date_dojang_record: String,
    pub dojang_best_time: i32,
}

pub async fn get_user_dojang(
    Extension(api_key): Extension<Arc<API>>,
    header: HeaderMap,
) -> Result<Json<Dojang>, (StatusCode, &'static str)> {
    // POST 요청 보내기
    let response = request_parser(api_key.clone(), header, "dojang").await;

    // 응답 결과 확인
    if response.status().is_success() {
        let user_dojang: Dojang = response
            .json()
            .await
            .expect("Failed to parse response JSON");

        Ok(Json(user_dojang))
    } else {
        Err((StatusCode::BAD_REQUEST, "Failed to fetch OCID"))
    }
}
