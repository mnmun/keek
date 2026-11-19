use getset::{Getters, MutGetters};
use ratwrap::{
    name, ratatui::layout::Constraint, retained::paragraph, wrapped,
};

use crate::interface;

const WHEEL: [char; 8] = ['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾']; // TODO check out https://docs.rs/ratatui/0.30.0/ratatui/symbols/enum.Marker.html

fn update_widget(model: &Model, widget: &mut paragraph::Guard<'_>) {
    widget.set_text(model.state.to_string());
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Model {
    index: usize,
    state: char,
}

impl Model {
    pub fn widget(&self) -> wrapped::Builder<interface::Name> {
        wrapped::Builder::new(
            paragraph::Builder::default().with_text(self.state.to_string()),
        )
        .with_name(name!(interface::Name, Base, Info, Wheel, Itself))
        .with_constraint(Constraint::Min(0))
    }

    pub fn edit<'a>(&'a mut self, widget: paragraph::Guard<'a>) -> Guard<'a> {
        Guard::new(self, widget)
    }
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_index(mut self, value: usize) -> Self {
        self.index = value;
        self
    }

    #[must_use = "method returns the modified value"]
    pub fn with_state(mut self, value: char) -> Self {
        self.state = value;
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_index(&mut self, value: usize) -> &mut Self {
        self.index = value;
        self
    }

    pub fn set_state(&mut self, value: char) -> &mut Self {
        self.state = value;
        self
    }
}

#[derive(Debug, Getters, MutGetters)]
pub struct Builder {
    index: usize,
    state: char,
}

impl Default for Builder {
    fn default() -> Self {
        let index = 0;
        let state = WHEEL[index];

        Self { index, state }
    }
}

impl Builder {
    pub fn build(self) -> Model {
        Model {
            index: self.index,
            state: self.state,
        }
    }
}

pub struct Guard<'a> {
    inner: &'a mut Model,
    widget: paragraph::Guard<'a>,
    dirty: bool,
}

impl<'a> Guard<'a> {
    pub fn new(inner: &'a mut Model, widget: paragraph::Guard<'a>) -> Self {
        Self {
            inner,
            widget,
            dirty: false,
        }
    }
}

impl Guard<'_> {
    pub fn spin(&mut self) {
        self.dirty = true;
        self.inner.index += 1;
        self.inner.index %= WHEEL.len();
        self.inner.state = WHEEL[self.inner.index];
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        if !self.dirty {
            return;
        }

        update_widget(self.inner, &mut self.widget);
    }
}
