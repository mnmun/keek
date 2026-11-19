use getset::{Getters, MutGetters};
use ratwrap::{
    container, name,
    prelude::*,
    ratatui::{
        layout::{Constraint, Direction},
        style::Style,
    },
    retained::paragraph,
    viewport, wrapped,
};

use crate::{
    clip_string,
    interface::{self, MIN_COLUMN_WIDTH},
};

fn clean(
    model: &Model,
    widget: &mut impl container::CompositionMethods<interface::Name>,
) {
    let width = match widget.inner().constraint() {
        Constraint::Length(width) => *width,
        _ => unreachable!(),
    };

    widget
        .edit_items_placement(|_, _| true)
        .into_iter()
        .zip(model.entries.iter())
        .for_each(|(entry, (_, content))| {
            let mut as_wrapped = entry.as_wrapped().unwrap();
            let as_paragraph =
                as_wrapped.widget_mut().as_paragraph_mut_ref().unwrap();

            as_paragraph.edit().set_text(clip_string(
                content.clone(),
                "...",
                width as usize,
            ));
        });
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Model {
    entries: Vec<(Box<[usize]>, String)>,
}

impl Model {
    pub fn widget(&self) -> container::Builder<interface::Name> {
        container::Builder::default()
            .with_name(name!(
                interface::Name,
                Base,
                Page,
                Overlay,
                Print,
                Itself
            ))
            .with_direction(Direction::Vertical)
            .with_viewport(viewport::Builder::default().with_viewport(
                viewport::Data::SplitArea {
                    size: 1,
                    control: viewport::Control::StickToEnd,
                },
            ))
            .with_constraint(Constraint::Length(MIN_COLUMN_WIDTH))
            .with_items(self.entries.iter().map(|(_, content)| {
                wrapped::Builder::new(paragraph::Builder::default().with_text(
                    clip_string(
                        content.clone(),
                        "...",
                        MIN_COLUMN_WIDTH as usize,
                    ),
                ))
                .with_transparent(false)
                .with_constraint(Constraint::Length(1))
            }))
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
    entries: Vec<(Box<[usize]>, String)>,
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_entries<T>(mut self, value: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<(Box<[usize]>, String)>,
    {
        self.entries = value.into_iter().map(|value| value.into()).collect();
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_entries<T>(&mut self, value: T) -> &mut Self
    where
        T: IntoIterator,
        T::Item: Into<(Box<[usize]>, String)>,
    {
        self.entries = value.into_iter().map(|value| value.into()).collect();
        self
    }
}

impl Builder {
    pub fn build(self) -> Model {
        Model {
            entries: self.entries,
        }
    }
}

pub trait AppearanceMethods {}

pub trait CompositionMethods: AppearanceMethods {
    fn select_entry(&mut self, entry: (Box<[usize]>, String)) -> &mut Self;
}

pub trait PlacementMethods: CompositionMethods {
    fn set_width(&mut self, value: u16, limit: u16) -> &mut Self;
    fn inc_width(&mut self, limit: u16) -> &mut Self;
    fn dec_width(&mut self, limit: u16) -> &mut Self;
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

#[allow(unused)]
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

impl AppearanceMethods for AppearanceGuard<'_> {}

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

impl CompositionGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }
}

impl AppearanceMethods for CompositionGuard<'_> {}

impl CompositionMethods for CompositionGuard<'_> {
    fn select_entry(&mut self, entry: (Box<[usize]>, String)) -> &mut Self {
        let width = match self.widget.inner().constraint() {
            Constraint::Length(width) => *width,
            _ => unreachable!(),
        };

        if let Some(index) = self.inner.entries.iter().position(|e| e == &entry)
        {
            self.inner.entries.remove(index);
            self.widget.items_mut().remove(index);
        } else {
            self.widget.items_mut().push(
                wrapped::Builder::new(
                    paragraph::Builder::default()
                        .with_text(clip_string(
                            entry.1.clone(),
                            "...",
                            width as usize,
                        ))
                        .with_style(Style::default().on_red()),
                )
                .with_transparent(false)
                .with_constraint(Constraint::Length(1))
                .build()
                .into(),
            );
            self.inner.entries.push(entry);
        }

        self
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
}

impl AppearanceMethods for PlacementGuard<'_> {}

impl CompositionMethods for PlacementGuard<'_> {
    fn select_entry(&mut self, entry: (Box<[usize]>, String)) -> &mut Self {
        let width = match self.widget.inner().constraint() {
            Constraint::Length(width) => *width,
            _ => unreachable!(),
        };

        if let Some(index) = self.inner.entries.iter().position(|e| e == &entry)
        {
            self.inner.entries.remove(index);
            self.widget.items_mut().remove(index);
        } else {
            self.widget.items_mut().push(
                wrapped::Builder::new(
                    paragraph::Builder::default()
                        .with_text(clip_string(
                            entry.1.clone(),
                            "...",
                            width as usize,
                        ))
                        .with_style(Style::default().on_red()),
                )
                .with_transparent(false)
                .with_constraint(Constraint::Length(1))
                .build()
                .into(),
            );
            self.inner.entries.push(entry);
        }

        self
    }
}

impl PlacementMethods for PlacementGuard<'_> {
    fn set_width(&mut self, value: u16, limit: u16) -> &mut Self {
        self.dirty = true;
        self.widget.set_constraint(Constraint::Length(
            value.clamp(MIN_COLUMN_WIDTH, limit),
        ));
        self
    }

    fn inc_width(&mut self, limit: u16) -> &mut Self {
        self.dirty = true;
        let width = match self.widget.inner().constraint() {
            Constraint::Length(width) => *width,
            _ => unreachable!(),
        };
        self.set_width(width.saturating_add(1), limit)
    }

    fn dec_width(&mut self, limit: u16) -> &mut Self {
        self.dirty = true;
        let width = match self.widget.inner().constraint() {
            Constraint::Length(width) => *width,
            _ => unreachable!(),
        };
        self.set_width(width.saturating_sub(1), limit)
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
