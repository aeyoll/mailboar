use std::fs;
use std::sync::OnceLock;

static ASSETS_PATH: OnceLock<String> = OnceLock::new();

pub fn set_assets_path(path: &String) {
    ASSETS_PATH
        .set(path.to_string())
        .expect("Assets path already set");
}

fn get_assets_path() -> &'static str {
    ASSETS_PATH.get().expect("Assets path not set")
}

/// Read an asset manifest and return the correct filename from this manifest
pub fn get_asset_path(path: &str) -> String {
    let static_dir: &str = "static";
    let assets_path = get_assets_path();
    let manifest_file_name: &str = "assets-manifest.json";

    let manifest = format!("{}/{}", assets_path, manifest_file_name);

    let manifest_content = match fs::read_to_string(manifest) {
        Ok(manifest_content) => manifest_content,
        Err(_error) => panic!("Impossible to read manifest file."),
    };

    let json: serde_json::Value = match serde_json::from_str(manifest_content.as_str()) {
        Ok(json) => json,
        Err(_error) => panic!("Impossible to parse manifest file."),
    };

    let asset_path = match json.get(path) {
        Some(asset_path) => asset_path.as_str().unwrap(),
        None => path,
    };

    let public_path = format!("/{}/{}", static_dir, asset_path);
    public_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_assets_path() {
        let assets_path = "assets";
        set_assets_path(&assets_path.to_string());
        assert_eq!(get_assets_path(), assets_path);
    }
}
