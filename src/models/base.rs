use getset::{Getters, MutGetters};
use ratwrap::{container, name, prelude::*, ratatui::layout::Direction};

use crate::{
    interface,
    models::{info, page},
};

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Model {
    page: page::Model,
    info: info::Model,
}

impl Model {
    pub fn widget(&self) -> container::Builder<interface::Name> {
        container::Builder::default()
            .with_name(name!(interface::Name, Base, Itself))
            .with_direction(Direction::Vertical)
            .push_item(self.page.widget())
            .push_item(self.info.widget())
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

#[derive(Debug, Default, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Builder {
    page: page::Builder,
    info: info::Builder,
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_page(mut self, value: impl Into<page::Builder>) -> Self {
        self.page = value.into();
        self
    }

    #[must_use = "method returns the modified value"]
    pub fn with_info(mut self, value: impl Into<info::Builder>) -> Self {
        self.info = value.into();
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_page(&mut self, value: impl Into<page::Builder>) -> &mut Self {
        self.page = value.into();
        self
    }

    pub fn set_info(&mut self, value: impl Into<info::Builder>) -> &mut Self {
        self.info = value.into();
        self
    }
}

impl Builder {
    pub fn build(self) -> Model {
        Model {
            page: self.page.build(),
            info: self.info.build(),
        }
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

impl AppearanceMethods for AppearanceGuard<'_> {}

#[allow(unused)]
impl AppearanceGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_page_appearance<
        T,
        F: FnOnce(page::AppearanceGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let page = self
            .widget
            .edit_items_appearance(|_, item| {
                *item.name() == Some(name!(interface::Name, Base, Page, Itself))
            })
            .pop()
            .unwrap()
            .as_stack()
            .unwrap();

        f(self.inner.page.edit_appearance(page))
    }

    pub fn edit_page_composition<
        T,
        F: FnOnce(page::CompositionGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let page = self
            .widget
            .edit_items_composition(|_, item| {
                *item.name() == Some(name!(interface::Name, Base, Page, Itself))
            })
            .pop()
            .unwrap()
            .as_stack()
            .unwrap();

        f(self.inner.page.edit_composition(page))
    }

    pub fn edit_info_appearance<
        T,
        F: FnOnce(info::AppearanceGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let info = self
            .widget
            .edit_items_appearance(|_, item| {
                *item.name() == Some(name!(interface::Name, Base, Info, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.info.edit_appearance(info))
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

impl AppearanceMethods for CompositionGuard<'_> {}

impl CompositionMethods for CompositionGuard<'_> {}

#[allow(unused)]
impl CompositionGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_page_placement<T, F: FnOnce(page::PlacementGuard<'_>) -> T>(
        &mut self,
        f: F,
    ) -> T {
        let page = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name() == Some(name!(interface::Name, Base, Page, Itself))
            })
            .pop()
            .unwrap()
            .as_stack()
            .unwrap();

        f(self.inner.page.edit_placement(page))
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

impl AppearanceMethods for PlacementGuard<'_> {}

impl CompositionMethods for PlacementGuard<'_> {}

impl PlacementMethods for PlacementGuard<'_> {}

#[allow(unused)]
impl PlacementGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_page_placement<T, F: FnOnce(page::PlacementGuard<'_>) -> T>(
        &mut self,
        f: F,
    ) -> T {
        let page = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name() == Some(name!(interface::Name, Base, Page, Itself))
            })
            .pop()
            .unwrap()
            .as_stack()
            .unwrap();

        f(self.inner.page.edit_placement(page))
    }
}
