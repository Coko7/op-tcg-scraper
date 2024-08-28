# 👒 op-tcg-scraper

One Piece TCG data scraper written in Rust.

## Where is the data?

For practical reasons, data is kept in a separate repository: https://github.com/Coko7/op-tcg-data

## 🎴 Supported card fields

- [x] id
- [x] name
- [x] rarity
- [x] category
- [x] set_id (not serialized)

Images:
- [ ] thumb_url
- [x] img_url
- [ ] illustration
- [ ] illustrator_name

Data values:
- [x] colors
- [ ] number
- [x] life/cost
- [x] attributes
- [x] power
- [x] counter

Text:
- [ ] types
- [ ] effect
- [ ] trigger
- [ ] notes
- [ ] copyright

## 🗺️ Road Map

- [x] Fetch card sets data
- [x] Better error handling
- [x] Fetch cards data for each card set (wip)
- [ ] Get card data for all card sets
- [ ] Organize and save cards data as JSON to files
- [ ] Add logs
- [ ] Support more card fields
- [ ] Download card images as well
- [ ] Make it locale-agnostic to be able to download data from Japanese and other versions
- [ ] User friendly CLI
