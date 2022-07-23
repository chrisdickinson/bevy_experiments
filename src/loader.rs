use anyhow::{Error, Result};
use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    reflect::TypeUuid, prelude::{AddAsset, App},
};
use serde_json::Value;


pub struct MetadataPlugin;
impl bevy::app::Plugin for MetadataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<JSONLoader>();
        app.add_asset::<Metadata>();
    }
}

#[derive(Default)]
pub struct JSONLoader;

impl AssetLoader for JSONLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move { Ok(load_json(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["json"];
        EXTENSIONS
    }
}

#[derive(TypeUuid)]
#[uuid = "3868f1ee-003b-411a-9041-6f7cd1595e4d"]
pub struct Metadata(pub Value);

async fn load_json<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<(), Error> {
    let json: Value = serde_json::from_slice(bytes)?;
    load_context.set_default_asset(LoadedAsset::new(Metadata(json)));
    Ok(())
}
