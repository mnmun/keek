use ratwrap::{
    container, name,
    ratatui::{
        layout::{Constraint, Direction, Margin},
        style::{Color, Style},
    },
    retained::{self, block, paragraph},
    wrapped,
};

use crate::interface;

#[derive(Debug)]
pub struct Model;

impl Model {
    pub fn widget(&self) -> container::Builder<interface::Name> {
        let title =
            wrapped::Builder::new(
                paragraph::Builder::default()
                    .with_text(" Press ESC to close ")
                    .with_block(block::Builder::default().with_style(
                        Style::default().bg(Color::Rgb(60, 56, 54)),
                    )),
            )
            .with_constraint(Constraint::Length(1));

        let left_column = Constraint::Percentage(25);
        let right_column = Constraint::Percentage(75);

        let cursor_movements = container::Builder::default()
            .with_direction(Direction::Horizontal)
            .with_constraint(Constraint::Length(1))
            .with_margin(Margin {
                horizontal: 1,
                vertical: 0,
            })
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("↑/↓/←/→"),
                )
                .with_constraint(left_column),
            )
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("Cursor movement"),
                )
                .with_constraint(right_column),
            );

        let select = container::Builder::default()
            .with_direction(Direction::Horizontal)
            .with_constraint(Constraint::Length(1))
            .with_margin(Margin {
                horizontal: 1,
                vertical: 0,
            })
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("ENTER"),
                )
                .with_constraint(left_column),
            )
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("Select entry"),
                )
                .with_constraint(right_column),
            );

        let column_width = container::Builder::default()
            .with_direction(Direction::Horizontal)
            .with_constraint(Constraint::Length(1))
            .with_margin(Margin {
                horizontal: 1,
                vertical: 0,
            })
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("SHIFT + ←/→"),
                )
                .with_constraint(left_column),
            )
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("Column width"),
                )
                .with_constraint(right_column),
            );

        let selected_width = container::Builder::default()
            .with_direction(Direction::Horizontal)
            .with_constraint(Constraint::Length(1))
            .with_margin(Margin {
                horizontal: 1,
                vertical: 0,
            })
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("CNTRL + ←/→"),
                )
                .with_constraint(left_column),
            )
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default()
                        .with_text("Width of the selected entries overlay"),
                )
                .with_constraint(right_column),
            );

        let show_selected = container::Builder::default()
            .with_direction(Direction::Horizontal)
            .with_constraint(Constraint::Length(1))
            .with_margin(Margin {
                horizontal: 1,
                vertical: 0,
            })
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default().with_text("SPACE"),
                )
                .with_constraint(left_column),
            )
            .push_item(
                wrapped::Builder::new(
                    paragraph::Builder::default()
                        .with_text("Show/hide selected entries overlay"),
                )
                .with_constraint(right_column),
            );

        let table = container::Builder::default()
            .with_direction(Direction::Vertical)
            .with_constraint(Constraint::Min(0));

        let popup = container::Builder::default()
            .with_direction(Direction::Vertical)
            .with_constraint(Constraint::Length(7))
            .with_block(
                block::Builder::default()
                    .with_style(Style::default().bg(Color::Rgb(80, 73, 69))),
            )
            .with_transparent(false);

        let vertical_split = container::Builder::default()
            .with_direction(Direction::Vertical)
            .with_constraint(Constraint::Length(52))
            .push_item(
                wrapped::Builder::new(retained::Builder::Spacer)
                    .with_constraint(Constraint::Min(0)),
            );

        let horizontal_split = container::Builder::default()
            .with_name(name!(interface::Name, Controls, Itself))
            .with_direction(Direction::Horizontal)
            .push_item(
                wrapped::Builder::new(retained::Builder::Spacer)
                    .with_constraint(Constraint::Min(0)),
            )
            .with_visible(false);

        horizontal_split.push_item(
            vertical_split.push_item(
                popup.push_item(title).push_item(
                    table
                        .push_item(cursor_movements)
                        .push_item(select)
                        .push_item(column_width)
                        .push_item(selected_width)
                        .push_item(show_selected),
                ),
            ),
        )
    }
}

#[derive(Debug, Default)]
pub struct Builder;

impl Builder {
    pub fn build(self) -> Model {
        Model
    }
}
