#[derive(Debug, Clone)]
pub enum Message {
    Previous,
    Next,

    // for text input
    InputChanged(String),

    // Actions
    Delete,
    Post,
}
