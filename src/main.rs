use std::{
    fs,
    path::PathBuf,
    thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use directories::BaseDirs;
use image::RgbaImage;
use resolved::{ResolveConfig, prelude::*};

const FOLDER_CLIPBOARD: &'static str = "Clipboard";
const CLOSE_TIME: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> ResolveResult<()> {
    let t = Instant::now();
    let resolve = Resolve::new_with_config(&ResolveConfig::keep_globals()).await?;

    let project = resolve
        .store(script! {
            local pm = self:GetProjectManager()
            return pm:GetCurrentProject()
        })
        .await?;

    let project_name: String = project.execute(script! { self:GetName() }).await?;
    let path = get_path(&project_name);

    let image_t = Instant::now();
    // get image
    let image_data = match arboard::Clipboard::new().unwrap().get_image() {
        Ok(data) => data,
        Err(_) => {
            println!("!! no image in clipboard !!\nclosing in 5 seconds");

            sleep(CLOSE_TIME);

            return Ok(());
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
    let image_e = image_t.elapsed();

    let media_pool = project
        .store(script! {
            return self:GetMediaPool()
        })
        .await?;
    let root_folder = media_pool.store(script! { self:GetRootFolder() }).await?;

    media_pool_folder(&root_folder, &media_pool).await?;

    let items = vec![path.to_str().unwrap().to_string().replace("\\", "/")];
    media_pool
        .execute::<()>(script! {
            self:ImportMedia($items)
            return nil
        })
        .await?;

    println!(
        "imported image into mediapool in (t:{:?}, i:{:?})\nclosing in 5 seconds",
        t.elapsed(),
        image_e
    );

    sleep(CLOSE_TIME);

    Ok(())
}

async fn media_pool_folder(root: &ItemRef, media_pool: &ItemRef) -> ResolveResult<()> {
    let mut maybe_folder: Option<ItemRef> = None;

    let sub_folders = root.store_list(script! { self:GetSubFolderList() }).await?;
    for item in &sub_folders.list() {
        let name: String = item.execute(script! { self:GetName() }).await?;
        if name == FOLDER_CLIPBOARD {
            maybe_folder = Some(item.clone());
            break;
        }
    }

    if let Some(folder) = maybe_folder {
        media_pool
            .execute::<bool>(script! { self:SetCurrentFolder(@folder) })
            .await?;
    } else {
        let folder = media_pool
            .store(script! { self:AddSubFolder(@root, $FOLDER_CLIPBOARD) })
            .await?;
        media_pool
            .execute::<bool>(script! { self:SetCurrentFolder(@folder) })
            .await?;

        println!("added new '{}' folder", FOLDER_CLIPBOARD);
    }

    Ok(())
}

/// `%APPDATA%/resolve_clipboards/{project}/{timestamp}.png`
fn get_path(project_name: &str) -> PathBuf {
    // use a custom dir if the user provided one otherwise appdata
    let mut base = if let Ok(custom_dir) = std::env::var("RESOLVE_CLIPBOARD_DIR") {
        let path = PathBuf::from(custom_dir);
        println!("Saving image to custom directory: '{}'", path.display());
        path
    } else {
        let mut base = BaseDirs::new().unwrap().data_dir().to_path_buf();
        base.push("resolve_clipboards");

        base
    };
    // then continue with always adding project name & checking files

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
