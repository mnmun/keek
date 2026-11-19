use std::ops::Deref;

use getset::{Getters, MutGetters};
use ratwrap::{
    name,
    prelude::*,
    ratatui::layout::{Rect, Size},
    stack::{self, Stack},
    traits::Renderable,
};

use crate::{
    Add, Remove,
    error::Error,
    models::{
        base,
        column::PlacementMethods as _,
        controls,
        overlay::{AppearanceMethods as _, PlacementMethods as _},
        print::{CompositionMethods as _, PlacementMethods as _},
        table::{
            AppearanceMethods as _, CompositionMethods as _,
            PlacementMethods as _,
        },
    },
    state::State,
    terminal::TerminalHandler,
};

pub const MIN_COLUMN_WIDTH: u16 = 20;

nestify::nest! {
    #[allow(unused)]*
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]*
    pub enum Name {
        Base(pub enum Base {
            Itself,
            Page(pub enum Page {
                Itself,
                Table(pub enum Table {
                    Itself,
                    Column(pub enum Column {
                        Itself,
                    }),
                }),
                Overlay(pub enum Overlay {
                    Itself,
                    Print(pub enum Print {
                        Itself,
                    }),
                })
            }),
            Info(pub enum Info {
                Itself,
                Wheel(pub enum Wheel {
                    Itself
                })
            })
        }),
        Controls(pub enum Controls {
            Itself,
        })
    }
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Interface {
    draw: bool,

    model: Model,
    view: Stack<Name>,
}

impl Deref for Interface {
    type Target = Stack<Name>;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl Interface {
    pub fn new(area: impl Into<Rect>) -> Self {
        let model = Builder::new(area).build();
        let view = model.widget().build();

        Self {
            draw: true,
            model,
            view,
        }
    }

    pub fn draw_on(
        &mut self,
        terminal: &mut TerminalHandler,
    ) -> Result<(), Error> {
        self.draw = false;

        terminal.draw(|frame| {
            Renderable::render(&self.view, frame.buffer_mut());
        })?;

        Ok(())
    }

    pub fn spin(&mut self) {
        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_info_appearance(|mut guard| {
                    guard.edit_wheel(|mut guard| {
                        guard.spin();
                    });
                });
            });
        });
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.edit_composition(|mut guard| {
            guard.set_area(Size::new(width, height));
        });
    }

    pub fn select_entry(&mut self, state: &State) {
        let hovered_entry = self.hovered_entry(&state.path[1..]);
        let capacity = self.table_inner_area().height as usize;

        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_page_appearance(|mut guard| {
                    let selected_entries =
                        guard.edit_overlay_appearance(|mut guard| {
                            guard.edit_print_composition(|mut guard| {
                                guard.select_entry(hovered_entry);
                                guard.inner().entries().clone()
                            })
                        });

                    guard.edit_table_appearance(|mut guard| {
                        guard.apply_style(
                            &state.path[1..],
                            capacity,
                            &selected_entries,
                        );
                    });
                });
            });
        });
    }

    pub fn inc_column_width(&mut self) {
        let limit = self.table_inner_area().width / 2;

        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_page_appearance(|mut guard| {
                    guard.edit_table_composition(|mut guard| {
                        guard.edit_columns_placement(|mut guard| {
                            guard.inc_width(limit);
                        });
                    });
                });
            });
        });
    }

    pub fn dec_column_width(&mut self) {
        let limit = self.table_inner_area().width / 2;

        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_page_appearance(|mut guard| {
                    guard.edit_table_composition(|mut guard| {
                        guard.edit_columns_placement(|mut guard| {
                            guard.dec_width(limit);
                        });
                    });
                });
            });
        });
    }

    pub fn inc_print_width(&mut self) {
        let limit = self.table_inner_area().width;

        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_page_appearance(|mut guard| {
                    guard.edit_overlay_composition(|mut guard| {
                        guard.edit_print_placement(|mut guard| {
                            guard.inc_width(limit);
                        })
                    });
                });
            })
        });
    }

    pub fn dec_print_width(&mut self) {
        let limit = self.table_inner_area().width;

        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_page_appearance(|mut guard| {
                    guard.edit_overlay_composition(|mut guard| {
                        guard.edit_print_placement(|mut guard| {
                            guard.dec_width(limit);
                        })
                    });
                });
            });
        });
    }

    pub fn change_columns(
        &mut self,
        remove: Option<Remove>,
        add: Option<Add>,
        state: &mut State,
    ) {
        let capacity = self.table_inner_area().height as usize;

        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_page_appearance(|mut guard| {
                    let selected_entries =
                        guard.inner().overlay().print().entries().clone();

                    guard.edit_table_composition(|mut guard| {
                        match (remove, add) {
                            (Some(remove), Some(Add { columns, update })) => {
                                state.update(update);
                                guard.update_columns(remove, columns);
                            }
                            (None, Some(Add { columns, update })) => {
                                state.update(update);
                                guard.extend_columns(columns);
                            }
                            (Some(remove), None) => {
                                guard.remove_columns(remove);
                            }
                            (None, None) => {}
                        }

                        guard.apply_style(
                            &state.path[1..],
                            capacity,
                            &selected_entries,
                        );
                    });
                });
            });
        });
    }

    pub fn show_controls(&mut self) {
        let mut interface = self.view.edit_appearance();

        let mut controls = interface
            .edit_items_appearance(|_, item| {
                *item.name() == Some(name!(Name, Controls, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        controls.set_visible(true);
    }

    pub fn hide_controls(&mut self) {
        let mut interface = self.view.edit_appearance();

        let mut controls = interface
            .edit_items_appearance(|_, item| {
                *item.name() == Some(name!(Name, Controls, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        controls.set_visible(false);
    }

    pub fn toggle_overlay(&mut self) {
        self.edit_appearance(|mut guard| {
            guard.edit_base_appearance(|mut guard| {
                guard.edit_page_appearance(|mut guard| {
                    guard.edit_overlay_appearance(|mut guard| {
                        if guard.inner().print().entries().is_empty() {
                            guard.set_visible(true);
                        } else {
                            guard.toggle();
                        }
                    });
                });
            });
        });
    }

    pub fn table_inner_area(&self) -> &Rect {
        let main = self
            .view
            .data
            .items()
            .iter()
            .find(|item| *item.name() == Some(name!(Name, Base, Itself)))
            .unwrap()
            .as_container_ref()
            .unwrap();

        let page = main
            .data
            .items()
            .iter()
            .find(|item| *item.name() == Some(name!(Name, Base, Page, Itself)))
            .unwrap()
            .as_stack_ref()
            .unwrap();

        let table = page
            .data
            .items()
            .iter()
            .find(|item| {
                *item.name() == Some(name!(Name, Base, Page, Table, Itself))
            })
            .unwrap()
            .as_container_ref()
            .unwrap();

        table.data.inner_area()
    }

    pub fn columns_len(&self) -> usize {
        let main = self
            .view
            .data
            .items()
            .iter()
            .find(|item| *item.name() == Some(name!(Name, Base, Itself)))
            .unwrap()
            .as_container_ref()
            .unwrap();

        let page = main
            .data
            .items()
            .iter()
            .find(|item| *item.name() == Some(name!(Name, Base, Page, Itself)))
            .unwrap()
            .as_stack_ref()
            .unwrap();

        let table = page
            .data
            .items()
            .iter()
            .find(|item| {
                *item.name() == Some(name!(Name, Base, Page, Table, Itself))
            })
            .unwrap()
            .as_container_ref()
            .unwrap();

        table.data.items().len()
    }

    fn edit_appearance<T, F: FnOnce(AppearanceGuard<'_>) -> T>(
        &mut self,
        f: F,
    ) -> T {
        self.draw = true;
        f(self.model.edit_appearance(self.view.edit_appearance()))
    }

    fn edit_composition<F: FnOnce(CompositionGuard<'_>)>(&mut self, f: F) {
        self.draw = true;
        f(self.model.edit_composition(self.view.edit_composition()));
    }

    fn hovered_entry(&self, path: &[usize]) -> (Box<[usize]>, String) {
        self.model
            .base()
            .page()
            .table()
            .columns()
            .get(path.len().checked_sub(1).unwrap())
            .unwrap()
            .entries()
            .get(*path.last().unwrap())
            .unwrap()
            .clone()
    }
}

fn clean(guard: &mut CompositionGuard) {
    let table_inner_area = guard
        .widget
        .inner()
        .peek_tree(&guard.inner.area)
        .iter()
        .find_map(|(name, inner, _)| {
            if *name == Some(name!(Name, Base, Page, Table, Itself)) {
                Some(*inner)
            } else {
                None
            }
        })
        .unwrap();

    guard.edit_base_placement(|mut guard| {
        guard.edit_page_placement(|mut guard| {
            guard.edit_table_placement(|mut guard| {
                guard.new_width(table_inner_area.width);
            });

            guard.edit_overlay_placement(|mut guard| {
                guard.new_width(table_inner_area.width);
            });
        });
    });

    guard.widget.set_outer_area(guard.inner.area);
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Model {
    area: Rect,

    base: base::Model,
    controls: controls::Model,
}

impl Model {
    pub fn widget(&self) -> stack::Builder<Name> {
        stack::Builder::default()
            .push_item(self.base.widget())
            .push_item(self.controls.widget())
            .with_outer_area(self.area)
    }

    pub fn edit_appearance<'a>(
        &'a mut self,
        widget: stack::AppearanceGuard<'a, Name>,
    ) -> AppearanceGuard<'a> {
        AppearanceGuard::new(self, widget)
    }

    pub fn edit_composition<'a>(
        &'a mut self,
        widget: stack::CompositionGuard<'a, Name>,
    ) -> CompositionGuard<'a> {
        CompositionGuard::new(self, widget)
    }
}

#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Builder {
    area: Rect,

    base: base::Builder,
    controls: controls::Builder,
}

#[allow(unused)]
impl Builder {
    #[must_use = "method returns the modified value"]
    pub fn with_base(mut self, value: impl Into<base::Builder>) -> Self {
        self.base = value.into();
        self
    }

    #[must_use = "method returns the modified value"]
    pub fn with_controls(
        mut self,
        value: impl Into<controls::Builder>,
    ) -> Self {
        self.controls = value.into();
        self
    }
}

#[allow(unused)]
impl Builder {
    pub fn set_base(&mut self, value: impl Into<base::Builder>) -> &mut Self {
        self.base = value.into();
        self
    }

    pub fn set_controls(
        &mut self,
        value: impl Into<controls::Builder>,
    ) -> &mut Self {
        self.controls = value.into();
        self
    }
}

impl Builder {
    pub fn new(area: impl Into<Rect>) -> Self {
        let area = area.into();

        Self {
            area,
            base: base::Builder::default(),
            controls: controls::Builder,
        }
    }

    pub fn build(self) -> Model {
        Model {
            area: self.area,
            base: self.base.build(),
            controls: self.controls.build(),
        }
    }
}

pub trait AppearanceMethods {}

#[allow(unused)]
pub trait CompositionMethods: AppearanceMethods {
    fn area_mut(&mut self) -> &mut Rect;
    fn set_area(&mut self, value: impl Into<Rect>) -> &mut Self;
}

#[allow(dead_code)]
fn assert_methods() {
    fn appearance_methods<T: AppearanceMethods>() {}
    appearance_methods::<AppearanceGuard<'_>>();
    appearance_methods::<CompositionGuard<'_>>();

    fn composition_methods<T: CompositionMethods>() {}
    composition_methods::<CompositionGuard<'_>>();
}

pub struct AppearanceGuard<'a> {
    inner: &'a mut Model,
    widget: stack::AppearanceGuard<'a, Name>,
}

impl<'a> AppearanceGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: stack::AppearanceGuard<'a, Name>,
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

    pub fn edit_base_appearance<
        T,
        F: FnOnce(base::AppearanceGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let mut base = self
            .widget
            .edit_items_appearance(|_, item| {
                *item.name() == Some(name!(Name, Base, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.base.edit_appearance(base))
    }

    pub fn edit_base_composition<
        T,
        F: FnOnce(base::CompositionGuard<'_>) -> T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let mut base = self
            .widget
            .edit_items_composition(|_, item| {
                *item.name() == Some(name!(Name, Base, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.base.edit_composition(base))
    }
}

pub struct CompositionGuard<'a> {
    inner: &'a mut Model,
    widget: stack::CompositionGuard<'a, Name>,
    dirty: bool,
}

impl<'a> CompositionGuard<'a> {
    fn new(
        inner: &'a mut Model,
        widget: stack::CompositionGuard<'a, Name>,
    ) -> Self {
        Self {
            inner,
            widget,
            dirty: false,
        }
    }
}

impl AppearanceMethods for CompositionGuard<'_> {}

impl CompositionMethods for CompositionGuard<'_> {
    fn area_mut(&mut self) -> &mut Rect {
        self.dirty = true;
        &mut self.inner.area
    }

    fn set_area(&mut self, value: impl Into<Rect>) -> &mut Self {
        self.dirty = true;
        self.inner.area = value.into();
        self
    }
}

#[allow(unused)]
impl CompositionGuard<'_> {
    pub fn inner(&self) -> &Model {
        self.inner
    }

    pub fn edit_base_placement<T, F: FnOnce(base::PlacementGuard<'_>) -> T>(
        &mut self,
        f: F,
    ) -> T {
        let mut base = self
            .widget
            .edit_items_placement(|_, item| {
                *item.name() == Some(name!(Name, Base, Itself))
            })
            .pop()
            .unwrap()
            .as_container()
            .unwrap();

        f(self.inner.base.edit_placement(base))
    }
}

impl Drop for CompositionGuard<'_> {
    fn drop(&mut self) {
        if !self.dirty {
            return;
        }

        clean(self);
    }
}
