use std::vec;

#[path = "engine.rs"]
mod engine;

use eframe::{egui::{self, Color32, Pos2, Rect, Sense, Shape, Stroke, StrokeKind}, epaint::RectShape};

struct GridApp {
    pub grid: Vec<Vec<bool>>,
}

impl GridApp {
    fn name() -> &'static str {
        "Game of Life in Rust"
    }
}

impl Default for GridApp {
    fn default() -> Self {
        GridApp { grid: vec![vec![]] }
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: There is a bug in here, which makes the window have some thin grey lines, when for example the window is split in half.
        // If the window is fullscreen, the lines go away.
        egui::CentralPanel::default().show(&ctx, |ui| {
            // screen_rect represents the entire area of the window.
            // The min and max of this structure have the units of logical pixels.
            let screen_rect = ctx.input(|i| i.screen_rect());
            let (_, painter) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::empty());

            // Use the number of rows and columns of the grid in order to create rectangles for each cell and draw them according to their value.
            let num_rows = self.grid.len();
            let num_columns = self.grid[0].len();

            // TODO: From this point on, I am sure things can be done much better, for example we can make each cell exactly a square and improve performance.

            // Let's calculate how big each cell should be, so that we fit everything on the screen.
            // We use the shortest side of the screen as our basis for width and height, so we always display square cells.
            let cell_width: f32;

            if screen_rect.max.x <= screen_rect.max.y {
                cell_width = screen_rect.max.x / num_columns as f32;
            } else {
                cell_width = screen_rect.max.y / num_rows as f32;
            }

            let mut shapes: Vec<Shape> = vec![];

            for i in 0..self.grid.len() {
                for j in 0..self.grid[i].len() {
                    let current_rect = Rect {
                        min: Pos2 {
                            x: i as f32 * cell_width,
                            y: j as f32 * cell_width,
                        },
                        max: Pos2 {
                            x: (i as f32 * cell_width) + cell_width,
                            y: (j as f32 * cell_width) + cell_width,
                        },
                    };

                    let current_color: Color32;

                    if self.grid[i][j] {
                        current_color = Color32::WHITE;
                    } else {
                        current_color = Color32::BLACK;
                    }

                    shapes.push(Shape::Rect(RectShape {
                        rect: current_rect,
                        corner_radius: 0.0.into(),
                        fill: current_color,
                        stroke: Stroke::default(),
                        stroke_kind: StrokeKind::Inside,
                        round_to_pixels: Some(true),
                        blur_width: 0.0,
                        brush: None
                    }));
                }
            }

            painter.extend(shapes);

            ctx.request_repaint();
        });

        // Make a step using the engine.
        self.grid = engine::step_simulation(&self.grid);
    }
}

pub(crate) fn run_gui(grid: Vec<Vec<bool>>) -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((1700.0, 1700.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        GridApp::name(),
        native_options,
        Box::new(|_ctx| Ok(Box::new(GridApp { grid: grid }))),
    )
}
