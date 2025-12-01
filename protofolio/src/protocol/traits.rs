//! Protocol trait definitions

/// Protocol trait for extensibility
pub trait Protocol {
    /// Protocol name
    fn name() -> &'static str;

    /// Protocol identifier
    fn identifier() -> &'static str;
}
