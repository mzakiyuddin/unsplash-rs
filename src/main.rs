use crate::utils::{create_download_link, get_all_photos, visit};
use log::info;

mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    const RANDOM_NUMBER: u16 = 200;

    let list_all_photos = get_all_photos("zakiego").await;
    let list_download_link = create_download_link(&list_all_photos).await;

    info!("Will download {} times", RANDOM_NUMBER);

    for i in 0..RANDOM_NUMBER {
        launch(&list_download_link, &list_download_link, &i, &RANDOM_NUMBER).await;
    }
}

async fn launch(
    list_all_photos: &Vec<String>,
    list_download_link: &Vec<String>,
    x: &u16,
    random_number: &u16,
) {
    let len = list_download_link.len();

    for i in 0..len {
        visit(&list_all_photos[i]).await.unwrap();
        visit(&list_download_link[i]).await.unwrap();
        info!(
            "Downloading - image {}/{} - {}/{}",
            i, len, x, random_number
        );
    }
}
