use getset::{Getters, MutGetters};
use ratwrap::{
    container, name,
    prelude::*,
    ratatui::{
        layout::{Constraint, Direction},
        style::{Color, Style},
    },
    retained::{self, block, paragraph},
    wrapped,
};

use crate::{interface, models::wheel};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Model {
    wheel: wheel::Model,
}

impl Model {
    pub fn widget(&self) -> container::Builder<interface::Name> {
        container::Builder::default()
            .with_name(name!(interface::Name, Base, Info, Itself))
            .with_direction(Direction::Horizontal)
            .with_constraint(Constraint::Length(1))
            .with_block(
                block::Builder::default()
                    .with_style(Style::default().bg(Color::Rgb(80, 73, 69))),
            )
            .push_item(
                wrapped::Builder::new(retained::Builder::Spacer)
                    .with_constraint(Constraint::Length(1)),
            )
            .push_item(self.wheel.widget())
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default()
                        .with_text("Press ? to open tooltip"),
                )
                .with_constraint(Constraint::Length(24)),
            )
    }

    pub fn edit_appearance<'a>(
        &'a mut self,
        widget: container::AppearanceGuard<'a, interface::Name>,
    ) -> AppearanceGuard<'a> {
        AppearanceGuard::new(self, widget)
    }
}

#[derive(Default, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Builder {
    wheel: wheel::Builder,
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_wheel(mut self, value: impl Into<wheel::Builder>) -> Self {
        self.wheel = value.into();
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_wheel(&mut self, value: impl Into<wheel::Builder>) -> &mut Self {
        self.wheel = value.into();
        self
    }
}

impl Builder {
    pub fn build(self) -> Model {
        Model {
            wheel: self.wheel.build(),
        }
    }
}

pub trait AppearanceMethods {
    // TODO убрать в реализацию AppearanceGuard
    fn edit_wheel<F: FnOnce(wheel::Guard<'_>)>(&mut self, f: F) -> &mut Self;
}

#[allow(dead_code)]
fn assert_methods() {
    fn appearance_methods<T: AppearanceMethods>() {}
    appearance_methods::<AppearanceGuard<'_>>();
}

pub struct AppearanceGuard<'a> {
    inner: &'a mut Model,
    widget: container::AppearanceGuard<'a, interface::Name>,
}

impl<'a> AppearanceGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: container::AppearanceGuard<'a, interface::Name>,
    ) -> Self {
        Self { inner, widget }
    }
}

impl AppearanceMethods for AppearanceGuard<'_> {
    fn edit_wheel<F: FnOnce(wheel::Guard<'_>)>(&mut self, f: F) -> &mut Self {
        let mut wheel = self
            .widget
            .edit_items_appearance(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Info, Wheel, Itself))
            })
            .pop()
            .unwrap()
            .as_wrapped()
            .unwrap();

        f(self
            .inner
            .wheel
            .edit(wheel.widget_mut().as_paragraph_mut_ref().unwrap().edit()));

        drop(wheel);

        self
    }
}
