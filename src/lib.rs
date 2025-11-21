pub mod admin;
pub mod frontend;
pub mod writing;

pub enum PluginCategories {
    Storyboards,
    Themes,
    Others,
    AssetManagement,
}

pub struct Metadata {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub author: Option<&'static str>,
    pub version: Option<&'static str>,
    pub categories: Option<&'static [&'static PluginCategories]>,
    pub tags: Option<&'static [&'static str]>,
}
