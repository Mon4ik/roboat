use reqwest::header;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use tokio::io::join;
use crate::{Client, RoboatError};
use crate::games::request_types::GamesDetailsResponse;

mod request_types;

const GAMES_DETAILS_API: &str = "https://games.roblox.com/v1/games?universeIds={universe_ids}";

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum AvatarType {
    MorphToR6,
    MorphToR15,

    #[default]
    PlayerChoice,
}

impl TryFrom<String> for AvatarType {
    type Error = RoboatError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "MorphToR6" => Ok(Self::MorphToR6),
            "MorphToR15" => Ok(Self::MorphToR15),
            "PlayerChoice" => Ok(Self::PlayerChoice),
            _ => Err(RoboatError::MalformedResponse)
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct GameDetail {
    #[serde(alias = "id")]
    pub universe_id: u64,
    pub root_place_id: u64,

    /// Translated game's name
    pub name: String,

    /// Translated game's description
    pub description: String,

    /// Original game's name
    pub source_name: String,

    /// Original game's description
    pub source_description: String,

    pub creator: GameCreator,

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

    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice
    pub universe_avatar_type: AvatarType,

    pub genre: String,
    pub is_all_genre: bool,

    pub is_favorited_by_user: bool,
    pub favorited_count: u64,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct GameCreator {
    pub id: u64,
    pub name: String,

    #[serde(alias = "type")]
    pub creator_type: String,

    #[serde(alias = "isRNVAccount")]
    pub is_rnv_account: bool,

    pub has_verified_badge: bool,
}

impl Client {
    /// Gets games' details using <https://groups.roblox.com/v1/groups/{group_id}/roles>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * If you pass valid roblosecurity, `is_favorited_by_user` would have right value
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const UNIVERSE_ID: u64 = 3717264063;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn games_details(&self, universe_ids: Vec<u64>) -> Result<Vec<GameDetail>, RoboatError> {
        let formatted_universe_ids = universe_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let formatted_url = GAMES_DETAILS_API
            .replace("{universe_ids}", &formatted_universe_ids);

        let cookie_string = self.cookie_string().unwrap_or(HeaderValue::from_static(""));

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie_string)
            .send().await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<GamesDetailsResponse>(response).await?;

        let mut games_details = Vec::new();

        for game_detail in raw.data {
            games_details.push(
                GameDetail {
                    universe_id: game_detail.id,
                    root_place_id: game_detail.root_place_id,
                    name: game_detail.name,
                    description: game_detail.description,
                    source_name: game_detail.source_name,
                    source_description: game_detail.source_description,
                    creator: GameCreator {
                        id: game_detail.creator.id,
                        name: game_detail.creator.name,
                        creator_type: game_detail.creator.creator_type,
                        is_rnv_account: game_detail.creator.is_rnv_account,
                        has_verified_badge: game_detail.creator.has_verified_badge,
                    },
                    price: game_detail.price,
                    allowed_gear_genres: game_detail.allowed_gear_genres,
                    allowed_gear_categories: game_detail.allowed_gear_categories,
                    is_genre_enforced: game_detail.is_genre_enforced,
                    copying_allowed: game_detail.copying_allowed,
                    playing: game_detail.playing,
                    visits: game_detail.visits,
                    max_players: game_detail.max_players,
                    created: game_detail.created,
                    updated: game_detail.updated,

                    universe_avatar_type: AvatarType::try_from(game_detail.universe_avatar_type)?,
                    genre: game_detail.genre,
                    is_all_genre: game_detail.is_all_genre,
                    is_favorited_by_user: game_detail.is_favorited_by_user,
                    favorited_count: game_detail.favorited_count,
                }
            )
        }

        Ok(games_details)
    }

    /// Gets games' details using <https://groups.roblox.com/v1/groups/{group_id}/roles>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * If you pass valid roblosecurity, `is_favorited_by_user` would have right value
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const UNIVERSE_ID: u64 = 3717264063;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn game_details(&self, universe_id: u64) -> Result<GameDetail, RoboatError> {
        let games_details = self.games_details(vec![universe_id]).await?;

        let game_detail = games_details
            .first()
            .ok_or(RoboatError::BadRequest)?;

        Ok(game_detail.clone())
    }
}