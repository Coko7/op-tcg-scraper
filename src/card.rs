use std::fmt;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::localizer::Localizer;

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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. `{}`", self.id, self.name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CardColor {
    Red,
    Green,
    Blue,
    Purple,
    Black,
    Yellow,
}

impl CardColor {
    pub fn parse(localizer: &Localizer, value: &str) -> Result<CardColor, anyhow::Error> {
        match localizer.match_color(value) {
            Some(key) => Ok(Self::from_str(&key)?),
            None => Err(anyhow!("Failed to match color `{}`", value)),
        }
    }

    pub fn from_str(value: &str) -> Result<CardColor, anyhow::Error> {
        match value {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            "purple" => Ok(Self::Purple),
            "black" => Ok(Self::Black),
            "yellow" => Ok(Self::Yellow),
            _ => Err(anyhow!("Unsupported color `{}`", value)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CardIllustration {
    Comic,
    Animation,
    Original,
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CardAttribute {
    Slash,
    Strike,
    Ranged,
    Special,
    Wisdom,
}

impl CardAttribute {
    pub fn parse(localizer: &Localizer, value: &str) -> Result<CardAttribute, anyhow::Error> {
        match localizer.match_attribute(value) {
            Some(key) => Ok(Self::from_str(&key)?),
            None => Err(anyhow!("Failed to match attribute `{}`", value)),
        }
    }

    pub fn from_str(value: &str) -> Result<CardAttribute, anyhow::Error> {
        match value {
            "slash" => Ok(Self::Slash),
            "strike" => Ok(Self::Strike),
            "ranged" => Ok(Self::Ranged),
            "special" => Ok(Self::Special),
            "wisdom" => Ok(Self::Wisdom),
            _ => Err(anyhow!("Unsupported attribute `{}`", value)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CardCategory {
    Leader,
    Character,
    Event,
    Stage,
    Don,
}

impl CardCategory {
    pub fn parse(localizer: &Localizer, value: &str) -> Result<CardCategory, anyhow::Error> {
        match localizer.match_category(value) {
            Some(key) => Ok(Self::from_str(&key)?),
            None => Err(anyhow!("Failed to match category `{}`", value)),
        }
    }

    pub fn from_str(value: &str) -> Result<CardCategory, anyhow::Error> {
        match value {
            "leader" => Ok(Self::Leader),
            "character" => Ok(Self::Character),
            "event" => Ok(Self::Event),
            "stage" => Ok(Self::Stage),
            "don" => Ok(Self::Don),
            _ => Err(anyhow!("Unsupported category `{}`", value)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CardRarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    SuperRare = 3,
    SecretRare = 4,
    Leader = 5,
    Special = 6,
    TreasureRare = 7,
    Promo = 8,
}

impl CardRarity {
    pub fn parse(localizer: &Localizer, value: &str) -> Result<CardRarity, anyhow::Error> {
        match localizer.match_rarity(value) {
            Some(key) => Ok(Self::from_str(&key)?),
            None => Err(anyhow!("Failed to match rarity `{}`", value)),
        }
    }

    pub fn from_str(value: &str) -> Result<CardRarity, anyhow::Error> {
        match value {
            "common" => Ok(Self::Common),
            "uncommon" => Ok(Self::Uncommon),
            "rare" => Ok(Self::Rare),
            "super_rare" => Ok(Self::SuperRare),
            "secret_rare" => Ok(Self::SecretRare),
            "leader" => Ok(Self::Leader),
            "special" => Ok(Self::Special),
            "treasure_rare" => Ok(Self::TreasureRare), // Supposedly added in OP07
            "promo" => Ok(Self::Promo),                // Promo cards (Ultra rare)
            _ => Err(anyhow!("Unsupported rarity `{}`", value)),
        }
    }
}
