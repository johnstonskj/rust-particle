// ------------------------------------------------------------------------------------------------
// Public Types ❱ Icons
// ------------------------------------------------------------------------------------------------

pub trait HasTextIcon {
    /* const */
    fn text_icon(&self) -> &'static str;
}

pub trait HasFontIcon {
    /* const */
    fn font_icon(&self) -> char;
}
