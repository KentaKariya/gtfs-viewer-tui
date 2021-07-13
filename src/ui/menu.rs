use tui::layout::{Rect, Constraint, Layout, Direction, Alignment};

use crate::ui::{App, get_generic_block, UIBlock, get_border_style};
use tui::backend::Backend;
use tui::Frame;
use std::error::Error;
use rusqlite::Connection;
use tui::widgets::{Paragraph, Borders, Block, ListItem, List};
use std::borrow::Borrow;
use tui::style::{Color, Style, Modifier};
use tui::text::Text;
use crate::db::GTFSDatabase;

pub fn build_menu<B>(
    app: &mut App, frame: &mut Frame<B>, db: &GTFSDatabase, root_area: Rect,
) -> Result<(), Box<dyn Error>>
    where B: Backend
{
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Max(100),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(root_area);

    frame.render_widget(get_search_field(app), layout[0]);
    render_station_list(app, frame, db, layout[1]);
    frame.render_widget(get_date_field(app), layout[2]);
    frame.render_widget(get_time_field(app), layout[3]);

    Ok(())
}

fn get_search_field(app: &App) -> Paragraph {
    let text = Text::from(app.station_list.trigger.borrow());  //todo right func?
    Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(get_border_style(app, UIBlock::SEARCH))
            .title("Search")
        )
        .alignment(Alignment::Left)
}

fn render_station_list<B>(
    app: &mut App, frame: &mut Frame<B>, db: &GTFSDatabase, area: Rect,
)
    where B: Backend
{
    if app.station_list.changed {
        app.station_list.widget.set_items(db.fetch_stations(&app.station_list.trigger).unwrap());
        app.station_list.changed = false;
    }

    let items: Vec<ListItem> = app.station_list.widget.items.iter()
        .map(|s| ListItem::new(s.name.as_ref()))
        .collect();

    let list = List::new(items)
        .block(get_generic_block(app, UIBlock::STATION, Some("Stations")))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">>")
        .highlight_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        );

    frame.render_stateful_widget(list, area, &mut app.station_list.widget.state);
}

fn get_date_field(app: &App) -> Paragraph {
    let text = Text::from(app.selected_dt.format("%Y-%m-%d").to_string());
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_style(get_border_style(app, UIBlock::DATE)))
        .alignment(Alignment::Center)
}

fn get_time_field(app: &App) -> Paragraph {
    let text = Text::from(app.selected_dt.format("%H:%M").to_string());
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_style(get_border_style(app, UIBlock::TIME)))
        .alignment(Alignment::Center)
}