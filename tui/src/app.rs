use serde::{Deserialize, Serialize};
use std::time::Duration;
use tui::widgets::ListState;

const BENCHMARKS: [Benchmark; 2] = [
    Benchmark {
        to: Server {
            name: "NorthAmerica-1",
            location: "New York City",
            coords: (40.71, -74.00),
        },
        from: Server {
            name: "Europe-1",
            location: "Paris",
            coords: (48.85, 2.35),
        },
        timeline: Timeline {
            dns_resolution: Duration::new(0, 101273355),
            tcp_handshake: Duration::new(0, 15834533),
            tls_handshake: Duration::new(0, 84542919),
            first_byte: Duration::new(0, 34033797),
            total: Duration::new(0, 235684604),
        },
    },
    Benchmark {
        to: Server {
            name: "SouthAmerica-1",
            location: "São Paulo",
            coords: (-23.54, -46.62),
        },
        from: Server {
            name: "Asia-1",
            location: "Singapore",
            coords: (1.35, 103.86),
        },
        timeline: Timeline {
            dns_resolution: Duration::new(0, 101273355),
            tcp_handshake: Duration::new(0, 15834533),
            tls_handshake: Duration::new(0, 84542919),
            first_byte: Duration::new(0, 34033797),
            total: Duration::new(0, 235684604),
        },
    },
];

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Timeline {
    pub dns_resolution: Duration,
    pub tcp_handshake: Duration,
    pub tls_handshake: Duration,
    pub first_byte: Duration,
    pub total: Duration,
}

#[derive(Debug, Serialize, Clone)]
pub struct Benchmark<'a> {
    pub from: Server<'a>,
    pub to: Server<'a>,
    pub timeline: Timeline,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub show_chart: bool,
    pub progress: f64,
    pub benchmarks: StatefulList<Benchmark<'a>>,
    pub servers: Vec<Server<'a>>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            show_chart: true,
            progress: 0.0,
            benchmarks: StatefulList::with_items(BENCHMARKS.to_vec()),
            servers: vec![
                Server {
                    name: "NorthAmerica-1",
                    location: "New York City",
                    coords: (40.71, -74.00),
                },
                Server {
                    name: "Europe-1",
                    location: "Paris",
                    coords: (48.85, 2.35),
                },
            ],
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.benchmarks.previous();
    }

    pub fn on_down(&mut self) {
        self.benchmarks.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}
