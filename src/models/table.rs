use getset::Getters;
use ratwrap::{
    container, name,
    prelude::*,
    ratatui::{
        layout::{Constraint, Direction},
        style::{Color, Style},
    },
    viewport,
};

use crate::{
    Remove,
    interface::{self, MIN_COLUMN_WIDTH},
    models::column::{self, PlacementMethods as _},
};

#[allow(unused)]
fn clean(
    inner: &mut Model,
    widget: &mut impl container::CompositionMethods<interface::Name>,
) {
    widget.set_viewport(viewport::Data::SplitArea {
        size: widget.inner().items().first().map_or(
            MIN_COLUMN_WIDTH,
            |column| match column.constraint() {
                Constraint::Length(width) => *width,
                _ => unreachable!(),
            },
        ) as usize,
        control: viewport::Control::StickToEnd,
    });
}

#[derive(Getters, Debug)]
#[getset(get = "pub")]
pub struct Model {
    columns: Vec<column::Model>,
}

impl Model {
    pub fn widget(&self) -> container::Builder<interface::Name> {
        container::Builder::default()
            .with_name(name!(interface::Name, Base, Page, Table, Itself))
            .with_direction(Direction::Horizontal)
            .with_constraint(Constraint::Min(0))
            .with_viewport(viewport::Builder::default().with_viewport(
                viewport::Data::SplitArea {
                    size: MIN_COLUMN_WIDTH as usize,
                    control: viewport::Control::StickToEnd,
                },
            ))
            .with_items(
                self.columns
                    .iter()
                    .map(|column| column.widget(MIN_COLUMN_WIDTH)),
            )
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

#[derive(Default, Debug)]
pub struct Builder {
    columns: Vec<column::Builder>,
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_columns<T>(mut self, value: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<column::Builder>,
    {
        self.columns = value.into_iter().map(|value| value.into()).collect();
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_columns<T>(&mut self, value: T) -> &mut Self
    where
        T: IntoIterator,
        T::Item: Into<column::Builder>,
    {
        self.columns = value.into_iter().map(|value| value.into()).collect();
        self
    }
}

impl Builder {
    pub fn build(self) -> Model {
        Model {
            columns: self
                .columns
                .into_iter()
                .map(|column| column.build())
                .collect(),
        }
    }
}

pub trait AppearanceMethods {
    fn apply_style(
        &mut self,
        path: &[usize],
        capacity: usize,
        selected: &[(Box<[usize]>, String)],
    ) -> &mut Self;
}

pub trait CompositionMethods: AppearanceMethods {
    fn extend_columns(&mut self, columns: Box<[column::Model]>) -> &mut Self;
    fn remove_columns(&mut self, remove: Remove) -> &mut Self;
    fn update_columns(
        &mut self,
        remove: Remove,
        columns: Box<[column::Model]>,
    ) -> &mut Self;
}

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

#[allow(unused)]
impl AppearanceGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }
}

impl AppearanceMethods for AppearanceGuard<'_> {
    fn apply_style(
        &mut self,
        path: &[usize],
        capacity: usize,
        selected: &[(Box<[usize]>, String)],
    ) -> &mut Self {
        self.inner
            .columns
            .iter()
            .zip(self.widget.edit_items_appearance(|_, _| true))
            .enumerate()
            .for_each(|(column_index, (column, widget))| {
                column
                    .entries()
                    .iter()
                    .zip(
                        widget
                            .as_container()
                            .unwrap()
                            .edit_items_appearance(|_, _| true),
                    )
                    .enumerate()
                    .for_each(|(entry_index, (entry, widget))| {
                        widget
                            .as_wrapped()
                            .unwrap()
                            .widget_mut()
                            .as_paragraph_mut_ref()
                            .unwrap()
                            .edit()
                            .set_style(
                                if let Some(step) = path.get(column_index)
                                    && *step % capacity == entry_index
                                {
                                    if column_index == path.len() - 1 {
                                        Style::default()
                                            .bg(Color::Rgb(214, 94, 14))
                                    } else {
                                        Style::default()
                                            .bg(Color::Rgb(80, 73, 69))
                                    }
                                } else if selected.contains(entry) {
                                    Style::default().on_red()
                                } else {
                                    Style::default()
                                },
                            );
                    });
            });

        self
    }
}

pub struct CompositionGuard<'a> {
    inner: &'a mut Model,
    widget: container::CompositionGuard<'a, interface::Name>,
    dirty: bool,
}

impl<'a> CompositionGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: container::CompositionGuard<'a, interface::Name>,
    ) -> Self {
        Self {
            inner,
            widget,
            dirty: false,
        }
    }
}

#[allow(unused)]
impl CompositionGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_columns_placement<T, F: Fn(column::PlacementGuard<'_>) -> T>(
        &mut self,
        f: F,
    ) -> Vec<T> {
        self.dirty = true;

        self.inner
            .columns
            .iter_mut()
            .zip(self.widget.edit_items_placement(|_, _| true))
            .map(|(column, widget)| {
                f(column.edit_placement(widget.as_container().unwrap()))
            })
            .collect()
    }
}

impl AppearanceMethods for CompositionGuard<'_> {
    fn apply_style(
        &mut self,
        path: &[usize],
        capacity: usize,
        selected: &[(Box<[usize]>, String)],
    ) -> &mut Self {
        self.inner
            .columns
            .iter()
            .zip(self.widget.edit_items_placement(|_, _| true))
            .enumerate()
            .for_each(|(column_index, (column, widget))| {
                column
                    .entries()
                    .iter()
                    .zip(
                        widget
                            .as_container()
                            .unwrap()
                            .edit_items_placement(|_, _| true),
                    )
                    .enumerate()
                    .for_each(|(entry_index, (entry, widget))| {
                        widget
                            .as_wrapped()
                            .unwrap()
                            .widget_mut()
                            .as_paragraph_mut_ref()
                            .unwrap()
                            .edit()
                            .set_style({
                                if let Some(step) = path.get(column_index)
                                    && *step % capacity == entry_index
                                {
                                    if column_index == path.len() - 1 {
                                        Style::default()
                                            .bg(Color::Rgb(214, 94, 14))
                                    } else {
                                        Style::default()
                                            .bg(Color::Rgb(80, 73, 69))
                                    }
                                } else if selected.contains(entry) {
                                    Style::default().on_red()
                                } else {
                                    Style::default()
                                }
                            });
                    });
            });

        self
    }
}

impl CompositionMethods for CompositionGuard<'_> {
    fn extend_columns(&mut self, columns: Box<[column::Model]>) -> &mut Self {
        let width = self.widget.inner().items().first().map_or(
            MIN_COLUMN_WIDTH,
            |column| match column.constraint() {
                Constraint::Length(width) => *width,
                _ => unreachable!(),
            },
        );

        self.widget.items_mut().extend(
            columns
                .iter()
                .map(|model| model.widget(width).build().into()),
        );
        self.inner.columns.extend(columns);

        self
    }

    fn remove_columns(&mut self, remove: Remove) -> &mut Self {
        match remove {
            Remove::All => {
                self.inner.columns.clear();
                self.widget.items_mut().clear();
            }
            Remove::Pop { how_much } => {
                for _ in 0..how_much {
                    self.inner.columns.pop();
                    self.widget.items_mut().pop();
                }
            }
        }

        self
    }

    fn update_columns(
        &mut self,
        remove: Remove,
        columns: Box<[column::Model]>,
    ) -> &mut Self {
        let width = self.widget.inner().items().first().map_or(
            MIN_COLUMN_WIDTH,
            |column| match column.constraint() {
                Constraint::Length(width) => *width,
                _ => unreachable!(),
            },
        );

        match remove {
            Remove::All => {
                self.inner.columns.clear();
                self.widget.items_mut().clear();
            }
            Remove::Pop { how_much } => {
                for _ in 0..how_much {
                    self.inner.columns.pop();
                    self.widget.items_mut().pop();
                }
            }
        }

        self.widget.items_mut().extend(
            columns
                .iter()
                .map(|model| model.widget(width).build().into()),
        );
        self.inner.columns.extend(columns);

        self
    }
}

impl Drop for CompositionGuard<'_> {
    fn drop(&mut self) {
        if !self.dirty {
            return;
        }

        clean(self.inner, &mut self.widget);
    }
}

pub struct PlacementGuard<'a> {
    inner: &'a mut Model,
    widget: container::PlacementGuard<'a, interface::Name>,
    dirty: bool,
}

impl<'a> PlacementGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: container::PlacementGuard<'a, interface::Name>,
    ) -> Self {
        Self {
            inner,
            widget,
            dirty: false,
        }
    }
}

#[allow(unused)]
impl PlacementGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_columns_placement<T, F: Fn(column::PlacementGuard<'_>) -> T>(
        &mut self,
        f: F,
    ) -> Vec<T> {
        self.dirty = true;

        self.inner
            .columns
            .iter_mut()
            .zip(self.widget.edit_items_placement(|_, _| true))
            .map(|(column, widget)| {
                f(column.edit_placement(widget.as_container().unwrap()))
            })
            .collect()
    }
}

impl AppearanceMethods for PlacementGuard<'_> {
    fn apply_style(
        &mut self,
        path: &[usize],
        capacity: usize,
        selected: &[(Box<[usize]>, String)],
    ) -> &mut Self {
        self.inner
            .columns
            .iter()
            .zip(self.widget.edit_items_placement(|_, _| true))
            .enumerate()
            .for_each(|(column_index, (column, widget))| {
                column
                    .entries()
                    .iter()
                    .zip(
                        widget
                            .as_container()
                            .unwrap()
                            .edit_items_placement(|_, _| true),
                    )
                    .enumerate()
                    .for_each(|(entry_index, (entry, widget))| {
                        widget
                            .as_wrapped()
                            .unwrap()
                            .widget_mut()
                            .as_paragraph_mut_ref()
                            .unwrap()
                            .edit()
                            .set_style(
                                if let Some(step) = path.get(column_index)
                                    && *step % capacity == entry_index
                                {
                                    if column_index == path.len() - 1 {
                                        Style::default()
                                            .bg(Color::Rgb(214, 94, 14))
                                    } else {
                                        Style::default()
                                            .bg(Color::Rgb(80, 73, 69))
                                    }
                                } else if selected.contains(entry) {
                                    Style::default().on_red()
                                } else {
                                    Style::default()
                                },
                            );
                    });
            });

        self
    }
}

impl CompositionMethods for PlacementGuard<'_> {
    fn extend_columns(&mut self, columns: Box<[column::Model]>) -> &mut Self {
        let width = self.widget.inner().items().first().map_or(
            MIN_COLUMN_WIDTH,
            |column| match column.constraint() {
                Constraint::Length(width) => *width,
                _ => unreachable!(),
            },
        );

        self.widget.items_mut().extend(
            columns
                .iter()
                .map(|model| model.widget(width).build().into()),
        );
        self.inner.columns.extend(columns);

        self
    }

    fn remove_columns(&mut self, remove: Remove) -> &mut Self {
        match remove {
            Remove::All => {
                self.inner.columns.clear();
                self.widget.items_mut().clear();
            }
            Remove::Pop { how_much } => {
                for _ in 0..how_much {
                    self.inner.columns.pop();
                    self.widget.items_mut().pop();
                }
            }
        }

        self
    }

    fn update_columns(
        &mut self,
        remove: Remove,
        columns: Box<[column::Model]>,
    ) -> &mut Self {
        let width = self.widget.inner().items().first().map_or(
            MIN_COLUMN_WIDTH,
            |column| match column.constraint() {
                Constraint::Length(width) => *width,
                _ => unreachable!(),
            },
        );

        match remove {
            Remove::All => {
                self.inner.columns.clear();
                self.widget.items_mut().clear();
            }
            Remove::Pop { how_much } => {
                for _ in 0..how_much {
                    self.inner.columns.pop();
                    self.widget.items_mut().pop();
                }
            }
        }

        self.widget.items_mut().extend(
            columns
                .iter()
                .map(|model| model.widget(width).build().into()),
        );
        self.inner.columns.extend(columns);

        self
    }
}

impl PlacementMethods for PlacementGuard<'_> {
    fn new_width(&mut self, value: u16) -> &mut Self {
        self.dirty = true;

        let limit = value / 2;

        let width = self.widget.inner().items().first().map_or(
            MIN_COLUMN_WIDTH,
            |column| match column.constraint() {
                Constraint::Length(width) => *width,
                _ => unreachable!(),
            },
        );

        self.inner
            .columns
            .iter_mut()
            .zip(self.widget.edit_items_placement(|_, _| true))
            .for_each(|(column, widget)| {
                column
                    .edit_placement(widget.as_container().unwrap())
                    .set_width(width, limit);
            });

        self
    }
}

impl Drop for PlacementGuard<'_> {
    fn drop(&mut self) {
        if !self.dirty {
            return;
        }

        clean(self.inner, &mut self.widget);
    }
}
