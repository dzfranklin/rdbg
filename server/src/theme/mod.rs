mod import;
mod marketplace;

use std::collections::HashMap;

pub use marketplace::search;
use serde::Deserialize;
use tracing::warn;

#[derive(Debug)]
pub struct Theme {
    editor_background: Color,
    editor_foreground: Color,
    token_colors: StyleMap,
}

#[derive(Debug)]
pub struct StyleMap(HashMap<Scope, TokenStyle>);

impl StyleMap {
    pub fn get(&self, scope: &Scope) -> Option<&TokenStyle> {
        for scope in scope.all() {
            if let Some(style) = self.0.get(&scope) {
                return Some(style);
            }
        }
        None
    }

    pub fn insert(&mut self, scope: Scope, style: TokenStyle) {
        self.0.insert(scope, style);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Scope(Vec<String>);

impl Scope {
    pub fn parse(s: &str) -> Self {
        assert!(!s.contains(' '));
        assert!(s.is_ascii());
        Self(s.split('.').map(|s| s.to_owned()).collect())
    }

    pub fn all(&self) -> impl Iterator<Item = Scope> {
        // TODO: This could be an iterator to avoid making ones we don't need
        let mut all = Vec::new();
        for last in (0..self.0.len()).rev() {
            all.push(Self(self.0[..=last].to_vec()));
        }
        all.into_iter()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenStyle {
    foreground: Option<Color>,
    background: Option<Color>,
    font_style: Option<FontStyle>,
}

#[derive(Debug)]
pub enum FontStyle {
    Italic,
    Bold,
    Underlined,
    Unrecognized,
}

impl<'de> serde::de::Deserialize<'de> for FontStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de;

        struct FontVisitor;

        impl<'de> de::Visitor<'de> for FontVisitor {
            type Value = FontStyle;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("italic|bold|underlined")
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    "italic" => Ok(FontStyle::Italic),
                    "bold" => Ok(FontStyle::Bold),
                    "underlined" => Ok(FontStyle::Underlined),
                    other => {
                        warn!("Failed to parse font style `{}`", other);
                        Ok(FontStyle::Unrecognized)
                    }
                }
            }
        }

        deserializer.deserialize_str(FontVisitor)
    }
}

#[derive(Debug)]
pub enum Color {
    Value(u32),
    Inherit,
    Unrecognized,
}

impl Color {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        // See <https://stackoverflow.com/a/4801433/8334056>
        let color =
            (((r as u32) & 0x0ff) << 16) | (((g as u32) & 0x0ff) << 8) | ((b as u32) & 0x0ff);
        Self::Value(color)
    }
}

impl<'de> serde::de::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de;

        struct ColorVisitor;

        impl<'de> de::Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string containing a css color")
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    "inherit" => Ok(Color::Inherit),
                    color => match s.parse::<css_color_parser::Color>() {
                        Ok(color) => Ok(Color::from_rgb(color.r, color.g, color.b)),
                        Err(err) => {
                            warn!("Failed to parse theme color `{}`: {}", color, err);
                            Ok(Color::Unrecognized)
                        }
                    },
                }
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::theme::{search_request, SearchReply, SearchRequest};
    use pretty_assertions::assert_eq;

    #[ignore]
    #[tokio::test]
    async fn test_search() -> eyre::Result<()> {
        let req = SearchRequest {
            query: "Atom One Dark".into(),
            sort_by: search_request::SortBy::Relevance.into(),
            sort_order: search_request::SortOrder::Desc.into(),
            offset: 0,
            page_size: 10,
        };
        let resp = search(req).await?;
        assert!(!resp.themes.is_empty());
        Ok(())
    }

    #[test]
    fn test_scope_all() {
        let subject = Scope::parse("foo.bar.baq");
        let all = subject.all().collect::<Vec<_>>();
        assert_eq!(
            vec![
                Scope::parse("foo.bar.baq"),
                Scope::parse("foo.bar"),
                Scope::parse("foo")
            ],
            all
        );
    }
}
