use crate::*;
use std::iter;

// -- modules --

pub mod button;
pub use button::{Button, ButtonPressEvent};

pub mod canvas;
pub use canvas::Canvas;

pub mod checkbox;
pub use checkbox::Checkbox;

pub mod column;
pub use column::Column;

pub mod el;
pub use el::El;

pub mod image;
pub use image::Image;

pub mod label;
pub use label::Label;

pub mod link;
pub use link::{Link, NewTab};

pub mod paragraph;
pub use paragraph::Paragraph;

pub mod row;
pub use row::Row;

pub mod spacer;
pub use spacer::Spacer;

pub mod stack;
pub use stack::Stack;

pub mod text;
pub use text::Text;

pub mod text_area;
pub use text_area::TextArea;

pub mod text_input;
pub use text_input::{InputType, Placeholder, TextInput};

// --

pub mod raw_el;
pub use raw_el::{RawEl, RawHtmlEl, RawSvgEl, UpdateRawEl};

pub mod raw_text;
pub use raw_text::RawText;

// --

pub mod ability;
pub use ability::*;

// ------ Element ------

pub trait Element: IntoIterator<Item = Self> {
    fn into_raw_element(self) -> RawElement;
}

// ------ RawElement ------

pub enum RawElement {
    El(RawHtmlEl<web_sys::HtmlElement>),
    SvgEl(RawSvgEl<web_sys::SvgElement>),
    Text(RawText),
}

impl IntoDom for RawElement {
    fn into_dom(self) -> Dom {
        match self {
            RawElement::El(raw_el) => raw_el.into_dom(),
            RawElement::SvgEl(raw_svg_el) => raw_svg_el.into_dom(),
            RawElement::Text(raw_text) => raw_text.into_dom(),
        }
    }
}

impl Element for RawElement {
    fn into_raw_element(self) -> RawElement {
        self
    }
}

impl IntoIterator for RawElement {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

// ------ IntoDom ------

pub trait IntoDom {
    fn into_dom(self) -> Dom;
}

// ------ IntoElement ------

pub trait IntoElement<'a> {
    type EL: Element;
    fn into_element(self) -> Self::EL;
}

impl<'a, T: Element> IntoElement<'a> for T {
    type EL = T;
    fn into_element(self) -> Self::EL {
        self
    }
}

// ------ IntoOptionElement ------

pub trait IntoOptionElement<'a> {
    type EL: Element;
    fn into_option_element(self) -> Option<Self::EL>;
}

impl<'a, E: Element, T: IntoElement<'a, EL = E>> IntoOptionElement<'a> for Option<T> {
    type EL = E;
    fn into_option_element(self) -> Option<Self::EL> {
        self.map(|into_element| into_element.into_element())
    }
}

impl<'a, E: Element, T: IntoElement<'a, EL = E>> IntoOptionElement<'a> for T {
    type EL = E;
    fn into_option_element(self) -> Option<Self::EL> {
        Some(self.into_element())
    }
}
