use ggez::graphics::{Color, DrawMode, MeshBuilder, Rect};

pub struct Grid {
    pub rect: Rect,
    pub cell_size: (f32, f32),
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            rect: Rect::default(),
            cell_size: (0.0, 0.0),
            rows: 0,
            cols: 0,
        }
    }

    pub fn init(&mut self, r: usize, c: usize, size: (f32, f32)) {
        self.rect = Rect::new(size.0 / 6.0, 20.0, 800.0, 600.0);
        let c_size = (
            (self.rect.w / (c) as f32).round(),
            (self.rect.h / (r) as f32).round(),
        );
        self.cell_size = c_size;
        self.cols = c;
        self.rows = r;
    }

    pub fn build(&self) -> Option<MeshBuilder> {
        let mut mesh_builder = MeshBuilder::new();
        let _ = mesh_builder.rectangle(DrawMode::fill(), self.rect, Color::from_rgb(45, 49, 66));
        let w = (self.cols - 2) as f32 * self.cell_size.0;
        let h = self.rows as f32 * self.cell_size.1;

        for row in 0..self.rows {
            let y = self.rect.y + row as f32 * self.cell_size.1;
            let start_point = [self.rect.x, y];
            let end_point = [self.rect.x + w, y];
            let _ = mesh_builder.line(
                &[start_point, end_point],
                1.0,
                Color::from_rgba(128, 128, 128, 126),
            );
        }

        for col in 0..(self.cols - 1) {
            let x = self.rect.x + col as f32 * self.cell_size.0;
            let start_point = [x, self.rect.y];
            let end_point = [x, self.rect.y + h];
            let _ = mesh_builder.line(
                &[start_point, end_point],
                1.0,
                Color::from_rgba(128, 128, 128, 126),
            );
        }

        Some(mesh_builder)
    }
}
