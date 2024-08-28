# 👒 op-tcg-scraper

One Piece TCG data scraper written in Rust.

## Where is the data?

For practical reasons, data is kept in a separate repository: https://github.com/Coko7/op-tcg-data

## 🎴 Supported card fields

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub rarity: CardRarity,
    pub category: CardCategory,
    // pub number: i32,
    // #[serde(skip_serializing)]
    // pub set_id: String,
    // pub copyright: String,

    // Images
    pub img_url: String,
    // pub illustration: CardIllustration,
    // pub illustrator_name: String,

    // Gameplay
    pub colors: Vec<CardColor>,
    pub cost: Option<i32>, // Only Character, Event and Stage (called life for Leader)
    pub attributes: Vec<CardAttribute>, // Only Leader and Character
    pub power: Option<i32>, // Only Leader and Character
    pub counter: Option<i32>, // Only Character

    pub types: Vec<String>,
    pub effect: String,
    pub trigger: Option<String>,
    // pub notes: String,
}
```

## 🗺️ Road Map

- [x] Fetch card sets data
- [x] Better error handling
- [x] Fetch cards data for each card set (wip)
- [x] Get card data for all card sets
- [x] Organize and save cards data as JSON to files
- [x] Add logs
- [x] Support more card fields
- [x] Download card images as well
- [ ] Make it locale-agnostic to be able to download data from Japanese and other versions
- [ ] User friendly CLI
- [ ] Add tests
