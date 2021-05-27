use camino::Utf8PathBuf;
use eyre::eyre;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt,
    io::{Read, Seek},
    marker::PhantomData,
};

use super::Theme;

fn import(vsix: impl Read + Seek) -> eyre::Result<Theme> {
    let mut zip = zip::ZipArchive::new(vsix)?;

    let manifest = zip.by_name("extension.vsixmanifest")?;
    let manifest: VsixManifest = serde_xml_rs::from_reader(manifest)?;

    let packages: Vec<String> = manifest
        .assets
        .assets
        .into_iter()
        .filter_map(|asset| {
            if asset.ty == MANIFEST_ASSET_TY {
                asset.path
            } else {
                None
            }
        })
        .collect();

    let mut themes = Vec::new();
    for package_path in packages {
        let package = zip.by_name(&package_path)?;
        let package: Package = serde_json::from_reader(package)?;

        let package_path = Utf8PathBuf::from(package_path);
        let ext_path = package_path
            .parent()
            .ok_or_else(|| eyre!("Invalid package path"))?;

        for theme in package.contributes.themes {
            let path = ext_path.join(&theme.path);
            let path = path_clean::clean(path.as_str());
            eprintln!("{:?}", &path);
            let tm_theme = zip.by_name(&path)?;
            let tm_theme: TmTheme = serde_json::from_reader(tm_theme)?;
            themes.push(tm_theme);
        }
    }

    todo!("{:?}", themes);
}

// See <https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide>
// and <https://macromates.com/manual/en/language_grammars>

const MANIFEST_ASSET_TY: &str = "Microsoft.VisualStudio.Code.Manifest";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A TextMate theme
struct TmTheme {
    author: String,
    name: String,
    colors: HashMap<String, String>,
    token_colors: Vec<tm_theme::Token>,
}

mod tm_theme {
    use crate::theme::{Color, TokenStyle};

    use super::*;

    #[derive(Debug, Deserialize)]
    pub(super) struct Token {
        name: String,
        scope: Scope,
        settings: TokenStyle,
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum Scope {
        One(String),
        Multiple(Vec<String>),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct VsixManifest {
    assets: vsix_manifest::Assets,
}

mod vsix_manifest {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub(super) struct Assets {
        #[serde(rename = "Asset", default)]
        pub assets: Vec<Asset>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub(super) struct Asset {
        #[serde(rename = "Type")]
        pub ty: String,
        pub path: Option<String>,
    }
}

#[derive(Debug, Deserialize)]
struct Package {
    contributes: package::Contributes,
}

mod package {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub(super) struct Contributes {
        #[serde(default)]
        pub themes: Vec<Theme>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(super) struct Theme {
        pub label: String,
        pub ui_theme: String,
        pub path: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_import() -> eyre::Result<()> {
        let vsix = File::open("samples/theme-onedark.vsix")?;
        let theme = import(vsix)?;
        todo!("{:?}", theme)
    }
}
