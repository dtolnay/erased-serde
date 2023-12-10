use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Twitter<'a> {
    #[serde(borrow)]
    pub statuses: Vec<Status<'a>>,
    #[serde(borrow)]
    pub search_metadata: SearchMetadata<'a>,
}

pub type LongId = u64;
pub type ShortId = u32;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status<'a> {
    #[serde(borrow)]
    pub metadata: Metadata<'a>,
    pub created_at: &'a str,
    pub id: LongId,
    pub id_str: &'a str,
    pub text: String,
    pub source: String,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<LongId>,
    #[serde(borrow)]
    pub in_reply_to_status_id_str: Option<&'a str>,
    pub in_reply_to_user_id: Option<ShortId>,
    #[serde(borrow)]
    pub in_reply_to_user_id_str: Option<&'a str>,
    #[serde(borrow)]
    pub in_reply_to_screen_name: Option<&'a str>,
    #[serde(borrow)]
    pub user: User<'a>,
    pub geo: (),
    pub coordinates: (),
    pub place: (),
    pub contributors: (),
    #[serde(borrow)]
    pub retweeted_status: Option<Box<Status<'a>>>,
    pub retweet_count: u32,
    pub favorite_count: u32,
    #[serde(borrow)]
    pub entities: StatusEntities<'a>,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub lang: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata<'a> {
    pub result_type: &'a str,
    pub iso_language_code: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User<'a> {
    pub id: ShortId,
    pub id_str: &'a str,
    pub name: &'a str,
    pub screen_name: &'a str,
    pub location: &'a str,
    pub description: String,
    #[serde(borrow)]
    pub url: Option<&'a str>,
    #[serde(borrow)]
    pub entities: UserEntities<'a>,
    pub protected: bool,
    pub followers_count: u32,
    pub friends_count: u32,
    pub listed_count: u32,
    pub created_at: &'a str,
    pub favourites_count: u32,
    pub utc_offset: Option<i32>,
    #[serde(borrow)]
    pub time_zone: Option<&'a str>,
    pub geo_enabled: bool,
    pub verified: bool,
    pub statuses_count: u32,
    pub lang: &'a str,
    pub contributors_enabled: bool,
    pub is_translator: bool,
    pub is_translation_enabled: bool,
    pub profile_background_color: &'a str,
    pub profile_background_image_url: &'a str,
    pub profile_background_image_url_https: &'a str,
    pub profile_background_tile: bool,
    pub profile_image_url: &'a str,
    pub profile_image_url_https: &'a str,
    #[serde(borrow)]
    pub profile_banner_url: Option<&'a str>,
    pub profile_link_color: &'a str,
    pub profile_sidebar_border_color: &'a str,
    pub profile_sidebar_fill_color: &'a str,
    pub profile_text_color: &'a str,
    pub profile_use_background_image: bool,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub following: bool,
    pub follow_request_sent: bool,
    pub notifications: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntities<'a> {
    #[serde(borrow)]
    pub url: Option<UserUrl<'a>>,
    #[serde(borrow)]
    pub description: UserEntitiesDescription<'a>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserUrl<'a> {
    #[serde(borrow)]
    pub urls: Vec<Url<'a>>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Url<'a> {
    pub url: &'a str,
    pub expanded_url: &'a str,
    pub display_url: &'a str,
    pub indices: Indices,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserEntitiesDescription<'a> {
    #[serde(borrow)]
    pub urls: Vec<Url<'a>>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusEntities<'a> {
    #[serde(borrow)]
    pub hashtags: Vec<Hashtag<'a>>,
    pub symbols: [(); 0],
    #[serde(borrow)]
    pub urls: Vec<Url<'a>>,
    #[serde(borrow)]
    pub user_mentions: Vec<UserMention<'a>>,
    #[serde(borrow)]
    pub media: Option<Vec<Media<'a>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hashtag<'a> {
    pub text: &'a str,
    pub indices: Indices,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserMention<'a> {
    pub screen_name: &'a str,
    pub name: &'a str,
    pub id: ShortId,
    pub id_str: &'a str,
    pub indices: Indices,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Media<'a> {
    pub id: LongId,
    pub id_str: &'a str,
    pub indices: Indices,
    pub media_url: &'a str,
    pub media_url_https: &'a str,
    pub url: &'a str,
    pub display_url: &'a str,
    pub expanded_url: &'a str,
    #[serde(rename = "type")]
    pub media_type: &'a str,
    #[serde(borrow)]
    pub sizes: Sizes<'a>,
    pub source_status_id: Option<LongId>,
    #[serde(borrow)]
    pub source_status_id_str: Option<&'a str>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sizes<'a> {
    #[serde(borrow)]
    pub medium: Size<'a>,
    #[serde(borrow)]
    pub small: Size<'a>,
    #[serde(borrow)]
    pub thumb: Size<'a>,
    #[serde(borrow)]
    pub large: Size<'a>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Size<'a> {
    pub w: u16,
    pub h: u16,
    pub resize: &'a str,
}

pub type Indices = (u8, u8);

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchMetadata<'a> {
    pub completed_in: f32,
    pub max_id: LongId,
    pub max_id_str: &'a str,
    pub next_results: &'a str,
    pub query: &'a str,
    pub refresh_url: &'a str,
    pub count: u8,
    pub since_id: LongId,
    pub since_id_str: &'a str,
}
