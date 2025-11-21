#![allow(non_snake_case)]

pub enum ScreenplayKind {
    Narrator,
    Character,
    Cue,
}

pub struct ScreenplayContent {
    character: String,
    dialogue: String,
}

pub struct ScreenplayAttachments {
    file: Option<String>,
}

pub mod Screenplay {
    use super::{ScreenplayAttachments, ScreenplayContent, ScreenplayKind};

    pub fn delete_block(id: &str, force_delete: Option<bool>) {}
    pub fn delete_all_blocks() {}

    pub fn add_block(kind: &ScreenplayKind, contents: &ScreenplayContent) {}
    pub fn edit_block(id: &str, stuff_to_edit: &ScreenplayContent) {}

    pub fn embed_block(id: &str, attachments: &ScreenplayAttachments) {}
}

pub mod ScriptParser {}
