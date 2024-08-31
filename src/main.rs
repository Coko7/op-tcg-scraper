use anyhow::Result;
use chrono::Utc;
use localizer::Localizer;
use log::{error, info};

use op_scraper::OpTcgScraper;

mod card;
mod card_scraper;
mod card_set;
mod localizer;
mod op_data;
mod op_scraper;

fn main() -> Result<()> {
    env_logger::init();

    let localizer = Localizer::load_from_file("en")?;

    match scrap_all_cards(&localizer) {
        Ok(()) => (),
        Err(e) => {
            error!("failed to scrap cards data: {}", e);
        }
    }

    // download_all_images()?;
    Ok(())
}

// fn download_all_images() -> Result<(), anyhow::Error> {
//     info!("start massive download of OP TCG card images");
//
//     let host = "https://en.onepiece-cardgame.com";
//     let scraper = OpTcgScraper::new(host);
//
//     let card_sets = data::load_cards_data()?;
//
//     let mut count = 0;
//     for card_set in card_sets.iter() {
//         let cards = data::load_cards_for_set(&card_set.id)?;
//
//         for card in cards.iter() {
//             count += 1;
//             scraper.download_card_image(card)?;
//             println!("{}", card.id);
//         }
//     }
//
//     println!("SUMMARY: {} sets ; {} total cards", card_sets.len(), count);
//     Ok(())
// }

fn scrap_all_cards(localizer: &Localizer) -> Result<(), anyhow::Error> {
    info!("start OP TCG Scraper");

    let scraper = OpTcgScraper::new(&localizer);

    info!("fetching all card sets...");
    let start_time = Utc::now();

    let mut card_sets = scraper.fetch_all_card_sets()?;

    let total_sets = card_sets.len();
    info!("successfully fetched {} card sets!", total_sets);

    for (index, card_set) in card_sets.iter_mut().enumerate() {
        info!(
            "fetching cards for set {}/{}: `{}`",
            index, total_sets, card_set
        );

        let cards = scraper.fetch_all_cards(&card_set.id)?;
        info!(
            "successfully fetched {} cards for set: `{}`!",
            cards.len(),
            card_set
        );

        card_set.cards = cards;
    }

    info!("processed all {} card sets", card_sets.len());
    let end_time = Utc::now();

    let data = op_data::OnePieceTcgData {
        base_url: localizer.hostname.clone(),
        fetch_start_date: start_time,
        fetch_end_date: end_time,
        card_sets,
    };

    op_data::write_data(&data)?;
    Ok(())
}
