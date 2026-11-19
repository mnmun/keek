use getset::{Getters, MutGetters};
use ratwrap::{name, prelude::*, ratatui::layout::Constraint, stack};

use crate::{
    interface,
    models::{overlay, table},
};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Model {
    table: table::Model,
    overlay: overlay::Model,
}

impl Model {
    pub fn widget(&self) -> stack::Builder<interface::Name> {
        stack::Builder::default()
            .with_name(name!(interface::Name, Base, Page, Itself))
            .with_constraint(Constraint::Min(0))
            .push_item(self.table.widget())
            .push_item(self.overlay.widget())
    }

    pub fn edit_appearance<'a>(
        &'a mut self,
        widget: stack::AppearanceGuard<'a, interface::Name>,
    ) -> AppearanceGuard<'a> {
        AppearanceGuard::new(self, widget)
    }

    pub fn edit_composition<'a>(
        &'a mut self,
        widget: stack::CompositionGuard<'a, interface::Name>,
    ) -> CompositionGuard<'a> {
        CompositionGuard::new(self, widget)
    }

    pub fn edit_placement<'a>(
        &'a mut self,
        widget: stack::PlacementGuard<'a, interface::Name>,
    ) -> PlacementGuard<'a> {
        PlacementGuard::new(self, widget)
    }
}

#[derive(Default, Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Builder {
    table: table::Builder,
    overlay: overlay::Builder,
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_table(mut self, value: impl Into<table::Builder>) -> Self {
        self.table = value.into();
        self
    }

    #[must_use = "method returns the modified value"]
    pub fn with_overlay(mut self, value: impl Into<overlay::Builder>) -> Self {
        self.overlay = value.into();
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_table(&mut self, value: impl Into<table::Builder>) -> &mut Self {
        self.table = value.into();
        self
    }

    pub fn set_overlay(
        &mut self,
        value: impl Into<overlay::Builder>,
    ) -> &mut Self {
        self.overlay = value.into();
        self
    }
}

pub trait AppearanceMethods {}

pub trait CompositionMethods: AppearanceMethods {}

pub trait PlacementMethods: CompositionMethods {}

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

impl Builder {
    pub fn build(self) -> Model {
        Model {
            table: self.table.build(),
            overlay: self.overlay.build(),
        }
    }
}

pub struct AppearanceGuard<'a> {
    inner: &'a mut Model,
    widget: stack::AppearanceGuard<'a, interface::Name>,
}

impl<'a> AppearanceGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: stack::AppearanceGuard<'a, interface::Name>,
    ) -> Self {
        Self { inner, widget }
    }
}

impl AppearanceMethods for AppearanceGuard<'_> {}

impl AppearanceGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_table_appearance<
        T,
        F: FnOnce(table::AppearanceGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let table = self
            .widget
            .edit_items_appearance(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Table, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.table.edit_appearance(table))
    }

    pub fn edit_table_composition<
        T,
        F: FnOnce(table::CompositionGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let table = self
            .widget
            .edit_items_composition(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Table, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.table.edit_composition(table))
    }

    pub fn edit_overlay_appearance<
        T,
        F: FnOnce(overlay::AppearanceGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let overlay = self
            .widget
            .edit_items_appearance(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Overlay, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.overlay.edit_appearance(overlay))
    }

    pub fn edit_overlay_composition<
        T,
        F: FnOnce(overlay::CompositionGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let overlay = self
            .widget
            .edit_items_composition(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Overlay, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.overlay.edit_composition(overlay))
    }
}

#[allow(unused)]
pub struct CompositionGuard<'a> {
    inner: &'a mut Model,
    widget: stack::CompositionGuard<'a, interface::Name>,
}

impl<'a> CompositionGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: stack::CompositionGuard<'a, interface::Name>,
    ) -> Self {
        Self { inner, widget }
    }
}

impl AppearanceMethods for CompositionGuard<'_> {}

impl CompositionMethods for CompositionGuard<'_> {}

#[allow(unused)]
impl CompositionGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_table_placement<
        T,
        F: FnOnce(table::PlacementGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let table = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Table, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.table.edit_placement(table))
    }

    pub fn edit_overlay_placement<
        T,
        F: FnOnce(overlay::PlacementGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let overlay = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Overlay, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.overlay.edit_placement(overlay))
    }
}

pub struct PlacementGuard<'a> {
    inner: &'a mut Model,
    widget: stack::PlacementGuard<'a, interface::Name>,
}

impl AppearanceMethods for PlacementGuard<'_> {}

impl CompositionMethods for PlacementGuard<'_> {}

impl PlacementMethods for PlacementGuard<'_> {}

impl<'a> PlacementGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: stack::PlacementGuard<'a, interface::Name>,
    ) -> Self {
        Self { inner, widget }
    }
}

#[allow(unused)]
impl PlacementGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_table_placement<
        T,
        F: FnOnce(table::PlacementGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let table = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Table, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.table.edit_placement(table))
    }

    pub fn edit_overlay_placement<
        T,
        F: FnOnce(overlay::PlacementGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let overlay = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name()
                    == Some(name!(interface::Name, Base, Page, Overlay, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.overlay.edit_placement(overlay))
    }
}
