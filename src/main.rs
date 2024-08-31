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

    let localizer = Localizer::load_from_file("jp")?;

    match scrap_all_cards(&localizer) {
        Ok(()) => (),
        Err(error) => {
            error!("failed to scrap cards data: {}", error);
        }
    }

    // match download_all_images(&localizer) {
    //     Ok(()) => (),
    //     Err(error) => {
    //         error!("failed to download card images: {}", error);
    //     }
    // }

    Ok(())
}

fn download_all_images(localizer: &Localizer) -> Result<(), anyhow::Error> {
    info!("start massive download of OP TCG card images");

    let scraper = OpTcgScraper::new(&localizer);
    let data = op_data::load_data()?;

    for (set_idx, card_set) in data.card_sets.iter().enumerate() {
        info!(
            "downloading data for set `{}` ({}/{}), {} cards...",
            card_set.title,
            set_idx,
            data.card_sets.len(),
            card_set.cards.len()
        );

        for (card_idx, card) in card_set.cards.iter().enumerate() {
            scraper.download_card_image(card)?;
            info!(
                "({}) downloading card `{}` ({}/{})...",
                card_set.title,
                card.id,
                card_idx,
                card_set.cards.len()
            );

            println!("{}", card.id);
        }
    }

    info!(
        "downloaded images for: {} sets ; {} total cards",
        data.card_sets.len(),
        data.total_cards()
    );

    Ok(())
}

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
