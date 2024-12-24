use std::{
    fs,
    path::PathBuf,
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use directories::BaseDirs;
use image::RgbaImage;
use resolve_api::{Folder, MediaPool, Project};

const FOLDER_CLIPBOARD: &'static str = "Clipboard";
const CLOSE_TIME: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() {
    let project = Project::get_current_project().await.unwrap();

    let path = get_path(&project.get_name().await.unwrap());

    // get image
    let image_data = match arboard::Clipboard::new().unwrap().get_image() {
        Ok(data) => data,
        Err(_) => {
            println!("!! no image in clipboard !!\nclosing in 5 seconds");

            sleep(CLOSE_TIME);

            return;
        }
    };
    let image = RgbaImage::from_raw(
        image_data.width as u32,
        image_data.height as u32,
        (*image_data.into_owned_bytes()).to_vec(),
    )
    .unwrap();

    image
        .save_with_format(&path, image::ImageFormat::Png)
        .unwrap();

    let mediapool = MediaPool::new(&project);
    let mut root_folder = mediapool.get_root_folder().await.unwrap();

    media_pool_folder(&mut root_folder, &mediapool).await;

    // then import into that folder
    mediapool
        .import_media(&vec![path.to_str().unwrap().to_string().replace("\\", "/")])
        .await
        .unwrap();

    println!("imported image into mediapool\nclosing in 5 seconds");

    sleep(CLOSE_TIME);
}

/// `%APPDATA%/resolve_clipboards/{project}/{timestamp}.png`
fn get_path(project_name: &str) -> PathBuf {
    let mut base = BaseDirs::new().unwrap().data_dir().to_path_buf();

    base.push("resolve_clipboards");

    base.push(project_name);

    if !base.exists() {
        fs::create_dir_all(&base).unwrap();
    }

    base.push(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string()
            + ".png",
    );

    base
}

async fn media_pool_folder(root: &mut Folder, mediapool: &MediaPool) {
    // see if folder exists
    let mut clipboard_folder: Option<Folder> = None;
    for mut folder in root.get_subfolder_list().await.unwrap() {
        // if clipboard exists, set to that
        if folder.get_name().await.unwrap() == FOLDER_CLIPBOARD {
            clipboard_folder = Some(folder);
            break;
        }
    }

    // if it exists, go into, otherwise add it
    if let Some(folder) = clipboard_folder {
        mediapool.set_current_folder(&folder).await.unwrap();
    } else {
        let folder = mediapool
            .add_sub_folder(&root, &FOLDER_CLIPBOARD)
            .await
            .unwrap();
        mediapool.set_current_folder(&folder).await.unwrap();

        println!("added new '{}' folder", FOLDER_CLIPBOARD);
    }
}
