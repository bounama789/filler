use ggez::graphics::{self, Color, MeshBuilder, Rect};

pub struct Grid {
    pub cell_size: (f32,f32),
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cell_size: (0.0,0.0),
            rows: 0,
            cols: 0,
        }
    }

    pub fn init(&mut self, c_size: (f32,f32), r: usize, c: usize) {
        self.cell_size = c_size;
        self.cols = c;
        self.rows = r;
    }

    pub fn build(&self) -> Option<MeshBuilder> {
        let size = (800.0, 600.0);

        let (width, height) = size;

        let mut mesh_builder = MeshBuilder::new();

        for row in 0..=self.rows {
            let y = row as f32 * self.cell_size.1;
            let start_point = [0.0, y];
            let end_point = [width, y];
            mesh_builder.line(&[start_point, end_point], 1.0, Color::from_rgba(128, 128, 128, 126));
        }

        for col in 0..=self.cols {
            let x = col as f32 * self.cell_size.0;
            let start_point = [x, 0.0];
            let end_point = [x, height];
            mesh_builder.line(&[start_point, end_point], 1.0, Color::from_rgba(128, 128, 128, 126));
        }

        Some(mesh_builder)
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<graphics::GraphicsContext>,
    ) -> Option<graphics::Rect> {
        Some(Rect::new(
            0.0,
            0.0,
            self.cell_size.0 * self.cols as f32,
            self.cell_size.1 * self.rows as f32,
        ))
    }
}
