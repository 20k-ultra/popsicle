use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution},
    widgets::{Block, Borders, Gauge, List, ListItem},
    Frame,
};

pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    // Split UI such that the top can be the map with PoPs and bottom shows selected PoP info
    let area = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .direction(Direction::Vertical)
        .split(f.size());
    // Area to display a map with PoP list on the left
    let main_layout = Layout::default()
        .constraints([Constraint::Percentage(28), Constraint::Percentage(72)].as_ref())
        .direction(Direction::Horizontal)
        .split(area[0]);
    // Draw PoP list
    draw_pop_list(f, app, main_layout[0]);
    // Draw map
    draw_map(f, app, main_layout[1]);
    // Draw selected PoP details
    draw_details(f, app, area[1]);
}

fn draw_pop_list<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Draw benchmarks list
    let benchmarks: Vec<ListItem> = app
        .benchmarks
        .items
        .iter()
        .map(|i| {
            ListItem::new(vec![Spans::from(Span::raw(format!(
                "{} <--> {}",
                i.to.location, i.from.location
            )))])
        })
        .collect();
    let benchmarks = List::new(benchmarks)
        .block(Block::default().borders(Borders::ALL).title("Benchmarks"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(benchmarks, area, &mut app.benchmarks.state);
}

fn draw_map<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let map = Canvas::default()
        .block(Block::default().title("Popsicle").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.layer();
            // TODO - MAKE THIS PULL FROM app.benchmarks
            for (i, s1) in app.servers.iter().enumerate() {
                for s2 in &app.servers[i + 1..] {
                    ctx.draw(&Line {
                        x1: s1.coords.1,
                        y1: s1.coords.0,
                        y2: s2.coords.0,
                        x2: s2.coords.1,
                        color: Color::Yellow,
                    });
                }
            }
            for server in &app.servers {
                ctx.print(
                    server.coords.1,
                    server.coords.0,
                    Span::styled("X", Style::default().fg(Color::Green)),
                );
            }
        })
        .marker(if app.enhanced_graphics {
            symbols::Marker::Braille
        } else {
            symbols::Marker::Dot
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(map, area);
}

fn draw_details<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Benchmark Timeline");
    f.render_widget(block, area);

    // TODO - MAKE THIS PULL FROM app.benchmarks
    draw_bar(f, "DNS - 101 ms", 0.42, Color::LightMagenta, chunks[0]);
    draw_bar(f, "TCP - 15 ms", 0.06, Color::LightBlue, chunks[1]);
    draw_bar(f, "TLS - 84 ms", 0.35, Color::LightYellow, chunks[2]);
    draw_bar(f, "First Byte - 34 ms", 0.14, Color::LightCyan, chunks[3]);
    draw_bar(f, "Total - 235 ms", 1.0, Color::LightGreen, chunks[4]);
}

fn draw_bar<B>(f: &mut Frame<B>, label: &str, ratio: f64, color: Color, area: Rect)
where
    B: Backend,
{
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(color)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .ratio(ratio);
    f.render_widget(gauge, area);
}
