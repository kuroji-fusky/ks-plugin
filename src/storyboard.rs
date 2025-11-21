pub enum StoryboardKind {
    Character,
    Cue,
}

#[allow(non_snake_case)]
pub mod Storyboard {
    use crate::storyboard::StoryboardKind;

    pub fn delete_block(id: &str, force_delete: Option<bool>) {}
    pub fn add_block<T>(kind: &StoryboardKind, contents: T) {}
    pub fn edit_block<T>(id: &str, stuff_to_edit: T) {}
}
