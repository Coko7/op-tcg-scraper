# 👒 vegapull

A CLI utility to retrieve data for the One Piece Trading Cards Game (TCG).

## Where is the data?

For practical reasons, data is kept in a separate repository: https://github.com/Coko7/vegapull-records

## How to use?

1. Install / build from source:
```shell
$ git clone https://github.com/Coko7/vegapull.git
$ cd vegapull 
$ cargo build --release
```
2. Use
```shell
$ ./target/release/vegapull -h
Dynamically fetch data for the One Piece TCG from official sites.

Usage: vegapull [OPTIONS] <COMMAND>

Commands:
  packs  Get the list of all existing packs
  cards  Get all cards within the given pack
  help   Print this message or the help of the given subcommand(s)

Options:
  -l, --lang <LANGUAGE>  Language to use for the data [default: english] [possible values: english, japanese]
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -h, --help             Print help
```

3. You can also use the small bash script `pull_all.sh` that uses the `vegapull` CLI to download data for all existing packs:
```shell
$ bash pull_all.sh
```

## 🃏 Supported card fields

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
Fields have been named following the terms used in the official [rule book](https://en.onepiece-cardgame.com/pdf/rule_comprehensive.pdf)

## 🐛 Issues

When using `jp` locale to fetch data, the scraper will likely fail when handling `counter` or `colors` values for some cards.

## 🗺️ Road Map

- [x] Fetch card sets data
- [x] Better error handling
- [x] Fetch cards data for each card set (wip)
- [x] Get card data for all card sets
- [x] Organize and save cards data as JSON to files
- [x] Add logs
- [x] Support more card fields
- [x] Download card images as well
- [x] Make it locale-agnostic to be able to download data from Japanese and other versions
    - [ ] Handle problems with the `jp` version (inconsistent cards data on official site)
- [ ] Better configuration 
- [x] User friendly CLI
- [ ] Add tests
