use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GamesDetailsResponse {
    pub data: Vec<GameDetailRaw>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDetailRaw {
    pub id: u64,
    pub root_place_id: u64,

    pub name: String,
    pub description: String,
    pub source_name: String,
    pub source_description: String,

    pub creator: GameCreatorRaw,

    pub price: Option<i32>,

    pub allowed_gear_genres: Vec<String>,
    pub allowed_gear_categories: Vec<String>,
    pub is_genre_enforced: bool,
    pub copying_allowed: bool,

    pub playing: u64,
    pub visits: u64,
    pub max_players: u64,
    pub created: String,
    pub updated: String,

    pub studio_access_to_apis_allowed: bool,
    pub create_vip_servers_allowed: bool,

    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice
    pub universe_avatar_type: String,

    pub genre: String,
    pub is_all_genre: bool,

    pub is_favorited_by_user: bool,
    pub favorited_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCreatorRaw {
    pub id: u64,
    pub name: String,

    #[serde(alias = "type")]
    pub creator_type: String,

    #[serde(alias = "isRNVAccount")]
    pub is_rnv_account: bool,

    pub has_verified_badge: bool,
}