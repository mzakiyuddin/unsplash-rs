use log::info;
use scraper::{Html, Selector};
use std::collections::HashSet;

pub async fn get_all_photos(username: &str) -> Vec<String> {
    info!("Getting all photos from {}", username);

    let resp = reqwest::get(format!("http://unsplash.com/{username}")).await;
    let text = resp.unwrap().text().await.unwrap();

    let mut photos: HashSet<String> = HashSet::new();

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"a[itemprop="contentUrl""#).unwrap();
    for url in document.select(&selector) {
        let photo = url.value().attr("href").unwrap();
        photos.insert(format!("https://unsplash.com/{photo}"));
    }
    info!("Found {} photos", photos.len());

    Vec::from_iter(photos)
}

pub async fn create_download_link(list_all_photos: &Vec<String>) -> Vec<String> {
    info!("Creating download links");

    let mut list_download_link: HashSet<String> = HashSet::new();

    let mut i = 1;
    for photo in list_all_photos {
        let resp = reqwest::get(photo).await;
        let text = resp.unwrap().text().await.unwrap();
        let document = Html::parse_document(&text);
        let selector = Selector::parse(r#"a[title="Download photo"]"#).unwrap();

        for url in document.select(&selector) {
            let download_link = url.value().attr("href").unwrap();
            list_download_link.insert(download_link.to_string());
            info!("Create download link {}/{}", i, list_all_photos.len());
            i += 1;
        }
    }

    Vec::from_iter(list_download_link)
}

pub async fn visit(url: &String) -> Result<(), reqwest::Error> {
    reqwest::get(url).await?;
    Ok(())
}
