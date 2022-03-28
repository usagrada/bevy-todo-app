mod plugins;
pub use plugins::*;

use font_kit::{
    error::SelectionError,
    family_name::FamilyName,
    handle::Handle,
    properties::{Properties, Style},
    source::SystemSource,
};

/// Returns the default system font.
pub fn default_system_font() -> Result<Handle, SelectionError> {
    let source = SystemSource::new();
    let default_fonts = &[
        FamilyName::Title("arial".to_string()),
        FamilyName::SansSerif,
        FamilyName::Monospace,
        FamilyName::Fantasy,
    ];
    source.select_best_match(
        default_fonts,
        Properties::new().style(Style::Normal),
    )
}
