use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auctions {
    pub success: bool,
    pub page: i64,
    pub total_pages: i64,
    pub total_auctions: i64,
    pub last_updated: i64,
    pub auctions: Vec<Auction>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auction {
    pub uuid: String,
    pub auctioneer: String,
    #[serde(rename = "profile_id")]
    pub profile_id: String,
    pub coop: Vec<String>,
    pub start: i64,
    pub end: i64,
    #[serde(rename = "item_name")]
    pub item_name: String,
    #[serde(rename = "item_lore")]
    pub item_lore: String,
    pub extra: String,
    pub category: String,
    pub tier: String,
    #[serde(rename = "starting_bid")]
    pub starting_bid: i64,
    #[serde(rename = "item_bytes")]
    pub item_bytes: String,
    pub claimed: bool,
    #[serde(rename = "claimed_bidders")]
    pub claimed_bidders: Vec<serde_json::Value>,
    #[serde(rename = "highest_bid_amount")]
    pub highest_bid_amount: i64,
    #[serde(rename = "last_updated")]
    pub last_updated: i64,
    pub bin: bool,
    pub bids: Vec<Bid>,
    #[serde(rename = "item_uuid")]
    pub item_uuid: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    #[serde(rename = "auction_id")]
    pub auction_id: String,
    pub bidder: String,
    #[serde(rename = "profile_id")]
    pub profile_id: String,
    pub amount: i64,
    pub timestamp: i64,
}
