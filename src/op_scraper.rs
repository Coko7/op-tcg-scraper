use anyhow::{anyhow, Context};
use log::{debug, info};
use reqwest::blocking::Client;
use std::collections::HashMap;

use crate::{
    card::Card, card_scraper::CardScraper, card_set::CardSet, localizer::Localizer, op_data,
};

pub struct OpTcgScraper<'a> {
    base_url: String,
    localizer: &'a Localizer,
}

impl<'a> OpTcgScraper<'a> {
    // pub fn new2(base_url: &str) -> OpTcgScraper {
    //     OpTcgScraper {
    //         lo
    //         base_url: base_url.to_string(),
    //     }
    // }

    pub fn new(localizer: &Localizer) -> OpTcgScraper {
        OpTcgScraper {
            base_url: localizer.hostname.clone(),
            localizer,
        }
    }

    fn cardlist_endpoint(&self) -> String {
        format!("{}/{}", self.base_url, "cardlist")
    }

    fn get_img_full_url(&self, img_url: &str) -> String {
        let short_img_url = &img_url[3..];
        let full_url = format!("{}/{}", self.base_url, short_img_url);
        debug!("full url: {}", full_url);

        full_url
    }

    pub fn fetch_all_card_sets(&self) -> Result<Vec<CardSet>, anyhow::Error> {
        let url = self.cardlist_endpoint();
        info!("GET `{}`", url);

        let response = reqwest::blocking::get(url)?.text()?;

        info!("parsing HTML document");
        let document = scraper::Html::parse_document(&response);

        let sel = "div.seriesCol>select#series>option";
        info!("fetching series (card_sets) ({})...", sel);

        let series_selector = scraper::Selector::parse(sel).unwrap();

        let card_sets: Vec<CardSet> = document
            .select(&series_selector)
            .map(|x| CardSet::new(x))
            .filter(|cs| cs.id != "")
            .collect();

        info!("processed card_sets");
        Ok(card_sets)
    }

    pub fn fetch_all_cards(&self, card_set_id: &str) -> Result<Vec<Card>, anyhow::Error> {
        let url = self.cardlist_endpoint();
        info!("GET `{}`", url);

        let mut params = HashMap::new();
        params.insert("series", card_set_id);

        let client = Client::new();
        let response = client
            .get(self.cardlist_endpoint())
            .query(&params)
            .send()?
            .text()?;

        info!("parsing HTML document");
        let document = scraper::Html::parse_document(&response);

        let sel = "div.resultCol>a";
        info!("fetching cards for set `{}` ({})...", card_set_id, sel);

        let card_ids_selector = scraper::Selector::parse(sel).unwrap();

        let mut cards = Vec::new();
        for element in document.select(&card_ids_selector) {
            let card_id = element
                .attr("data-src")
                .context("expected `data-src` attr on <a>")?
                .to_string();

            let card_id = &card_id[1..];

            match CardScraper::create_card(&self.localizer, &document, &card_id) {
                Ok(card) => {
                    cards.push(card);
                }
                Err(error) => {
                    return Err(anyhow!(
                        "failed to scrap data about card `{}`: {}",
                        &card_id,
                        error
                    ))
                }
            };
        }

        info!("processed cards for set `{}`", card_set_id);
        Ok(cards)
    }

    pub fn download_card_image(&self, card: &Card) -> Result<(), anyhow::Error> {
        let full_url = self.get_img_full_url(&card.img_url);
        let img_file_path = op_data::compute_img_file_path(card)?;
        let mut file = std::fs::File::create(img_file_path).unwrap();

        reqwest::blocking::get(full_url)?.copy_to(&mut file)?;

        Ok(())
    }
}
