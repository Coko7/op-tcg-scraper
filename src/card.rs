use std::fmt;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

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
    pub fn from_str(value: &str) -> Result<CardColor, anyhow::Error> {
        match value.to_uppercase().as_str() {
            "RED" => Ok(Self::Red),
            "GREEN" => Ok(Self::Green),
            "BLUE" => Ok(Self::Blue),
            "PURPLE" => Ok(Self::Purple),
            "BLACK" => Ok(Self::Black),
            "YELLOW" => Ok(Self::Yellow),
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
    pub fn from_str(value: &str) -> Result<CardAttribute, anyhow::Error> {
        match value.to_uppercase().as_str() {
            "SLASH" => Ok(Self::Slash),
            "STRIKE" => Ok(Self::Strike),
            "RANGED" => Ok(Self::Ranged),
            "SPECIAL" => Ok(Self::Special),
            "WISDOM" => Ok(Self::Wisdom),
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
    pub fn from_str(value: &str) -> Result<CardCategory, anyhow::Error> {
        match value.to_uppercase().as_str() {
            "LEADER" => Ok(Self::Leader),
            "CHARACTER" => Ok(Self::Character),
            "EVENT" => Ok(Self::Event),
            "STAGE" => Ok(Self::Stage),
            "DON" => Ok(Self::Don),
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
    pub fn from_str(value: &str) -> Result<CardRarity, anyhow::Error> {
        match value.to_uppercase().as_str() {
            "C" => Ok(Self::Common),
            "UC" => Ok(Self::Uncommon),
            "R" => Ok(Self::Rare),
            "SR" => Ok(Self::SuperRare),
            "SEC" => Ok(Self::SecretRare),
            "L" => Ok(Self::Leader),
            "SP CARD" => Ok(Self::Special),
            "TR" => Ok(Self::TreasureRare), // Supposedly added in OP07
            "P" => Ok(Self::Promo),         // Promo cards (Ultra rare)
            _ => Err(anyhow!("Unsupported rarity `{}`", value)),
        }
    }
}
