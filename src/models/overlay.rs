use getset::{Getters, MutGetters};
use ratwrap::{
    container, name,
    prelude::*,
    ratatui::layout::{Constraint, Direction},
    retained, wrapped,
};

use crate::{
    interface,
    models::print::{self, PlacementMethods as _},
};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Model {
    print: print::Model,
}

impl Model {
    pub fn widget(&self) -> container::Builder<interface::Name> {
        container::Builder::default()
            .with_direction(Direction::Horizontal)
            .with_name(name!(interface::Name, Base, Page, Overlay, Itself))
            .push_item(
                wrapped::Builder::new(retained::Builder::Spacer)
                    .with_constraint(Constraint::Min(0)),
            )
            .push_item(self.print.widget())
    }

    pub fn edit_appearance<'a>(
        &'a mut self,
        widget: container::AppearanceGuard<'a, interface::Name>,
    ) -> AppearanceGuard<'a> {
        AppearanceGuard::new(self, widget)
    }

    pub fn edit_composition<'a>(
        &'a mut self,
        widget: container::CompositionGuard<'a, interface::Name>,
    ) -> CompositionGuard<'a> {
        CompositionGuard::new(self, widget)
    }

    pub fn edit_placement<'a>(
        &'a mut self,
        widget: container::PlacementGuard<'a, interface::Name>,
    ) -> PlacementGuard<'a> {
        PlacementGuard::new(self, widget)
    }
}

#[derive(Default, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Builder {
    print: print::Builder,
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_print(mut self, value: impl Into<print::Builder>) -> Self {
        self.print = value.into();
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_print(&mut self, value: impl Into<print::Builder>) -> &mut Self {
        self.print = value.into();
        self
    }
}

impl Builder {
    pub fn build(self) -> Model {
        Model {
            print: self.print.build(),
        }
    }
}

pub trait AppearanceMethods {
    fn set_visible(&mut self, value: bool) -> &mut Self;
    fn toggle(&mut self) -> &mut Self;
}

pub trait CompositionMethods: AppearanceMethods {}

pub trait PlacementMethods: CompositionMethods {
    fn new_width(&mut self, value: u16) -> &mut Self;
}

#[allow(dead_code)]
fn assert_methods() {
    fn appearance_methods<T: AppearanceMethods>() {}
    appearance_methods::<AppearanceGuard<'_>>();
    appearance_methods::<CompositionGuard<'_>>();
    appearance_methods::<PlacementGuard<'_>>();

    fn composition_methods<T: CompositionMethods>() {}
    composition_methods::<CompositionGuard<'_>>();
    composition_methods::<PlacementGuard<'_>>();

    fn placement_methods<T: PlacementMethods>() {}
    placement_methods::<PlacementGuard<'_>>();
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
    fn set_visible(&mut self, value: bool) -> &mut Self {
        self.widget.set_visible(value);
        self
    }

    fn toggle(&mut self) -> &mut Self {
        let value = !self.widget.inner().visible();
        self.set_visible(value);
        self
    }
}

#[allow(unused)]
impl AppearanceGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_print_appearance<
        T,
        F: FnOnce(print::AppearanceGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let print = self
            .widget
            .edit_items_appearance(|_, item| {
                *item.name()
                    == Some(name!(
                        interface::Name,
                        Base,
                        Page,
                        Overlay,
                        Print,
                        Itself
                    ))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.print.edit_appearance(print))
    }

    pub fn edit_print_composition<
        T,
        F: FnOnce(print::CompositionGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let print = self
            .widget
            .edit_items_composition(|_, item| {
                *item.name()
                    == Some(name!(
                        interface::Name,
                        Base,
                        Page,
                        Overlay,
                        Print,
                        Itself
                    ))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.print.edit_composition(print))
    }
}

pub struct CompositionGuard<'a> {
    inner: &'a mut Model,
    widget: container::CompositionGuard<'a, interface::Name>,
}

impl<'a> CompositionGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: container::CompositionGuard<'a, interface::Name>,
    ) -> Self {
        Self { inner, widget }
    }
}

impl AppearanceMethods for CompositionGuard<'_> {
    fn set_visible(&mut self, value: bool) -> &mut Self {
        self.widget.set_visible(value);
        self
    }

    fn toggle(&mut self) -> &mut Self {
        let value = !self.widget.inner().visible();
        self.set_visible(value);
        self
    }
}

impl CompositionMethods for CompositionGuard<'_> {}

#[allow(unused)]
impl CompositionGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_print_placement<
        T,
        F: FnOnce(print::PlacementGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let print = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name()
                    == Some(name!(
                        interface::Name,
                        Base,
                        Page,
                        Overlay,
                        Print,
                        Itself
                    ))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.print.edit_placement(print))
    }
}

pub struct PlacementGuard<'a> {
    inner: &'a mut Model,
    widget: container::PlacementGuard<'a, interface::Name>,
}

impl<'a> PlacementGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: container::PlacementGuard<'a, interface::Name>,
    ) -> Self {
        Self { inner, widget }
    }
}

impl AppearanceMethods for PlacementGuard<'_> {
    fn set_visible(&mut self, value: bool) -> &mut Self {
        self.widget.set_visible(value);
        self
    }

    fn toggle(&mut self) -> &mut Self {
        let value = !self.widget.inner().visible();
        self.set_visible(value);
        self
    }
}

impl CompositionMethods for PlacementGuard<'_> {}

impl PlacementMethods for PlacementGuard<'_> {
    fn new_width(&mut self, value: u16) -> &mut Self {
        let limit = value;

        let width = match self
            .widget
            .inner()
            .items()
            .iter()
            .find(|item| {
                *item.name()
                    == Some(name!(
                        interface::Name,
                        Base,
                        Page,
                        Overlay,
                        Print,
                        Itself
                    ))
            })
            .unwrap()
            .constraint()
        {
            Constraint::Length(width) => *width,
            _ => unreachable!(),
        };

        self.inner
            .print
            .edit_placement(
                self.widget
                    .edit_items_placement(|_, item| {
                        *item.name()
                            == Some(name!(
                                interface::Name,
                                Base,
                                Page,
                                Overlay,
                                Print,
                                Itself
                            ))
                    })
                    .pop()
                    .unwrap()
                    .as_container()
                    .unwrap(),
            )
            .set_width(width, limit);

        self
    }
}

#[allow(unused)]
impl PlacementGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_print_placement<
        T,
        F: FnOnce(print::PlacementGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let print = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name()
                    == Some(name!(
                        interface::Name,
                        Base,
                        Page,
                        Overlay,
                        Print,
                        Itself
                    ))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.print.edit_placement(print))
    }
}
