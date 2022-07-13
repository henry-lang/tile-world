use glium::{
    program::{ProgramCreationInput, SpirvEntryPoint, SpirvProgram},
    Display, Program,
};

static DEFAULT_VERT: &str = include_str!("default.vert");
static DEFAULT_FRAG: &str = include_str!("default.frag");

static TILES_VERT: &str = include_str!("tiles.vert");
static TILES_FRAG: &str = include_str!("tiles.frag");
static TILES_GEOM: &str = include_str!("tiles.geom");

pub struct Shaders {
    default: Program,
    tiles: Program,
}

impl Shaders {
    pub fn create_all(display: &Display) -> Self {
        let start = std::time::Instant::now();
        let default = Program::from_source(display, DEFAULT_VERT, DEFAULT_FRAG, None)
            .expect("create default shader");
        log::info!("Created default shader");
        let tiles = Program::from_source(display, TILES_VERT, TILES_FRAG, Some(TILES_GEOM))
            .expect("create tiles shader");
        log::info!("Created tiles shader");

        let end = std::time::Instant::now();

        log::info!("Shader creation took {}ms", (end - start).as_millis());

        Self { default, tiles }
    }

    pub fn default(&self) -> &Program {
        &self.default
    }

    pub fn tiles(&self) -> &Program {
        &self.tiles
    }
}
