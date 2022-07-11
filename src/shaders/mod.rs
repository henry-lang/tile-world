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
    // tiles: Program,
}

impl Shaders {
    pub fn create_all(display: &Display) -> Self {
        let start = std::time::Instant::now();
        let default = Program::from_source(display, DEFAULT_VERT, DEFAULT_FRAG, None).unwrap();
        log::info!("Created shader program: default");

        // let tiles = Program::new(
        // display,
        // ProgramCreationInput::SpirV(
        // SpirvProgram::from_vs_and_fs(
        // SpirvEntryPoint {
        // binary: TILES_VERT,
        // entry_point: "main",
        // },
        // SpirvEntryPoint {
        // binary: TILES_FRAG,
        // entry_point: "main",
        // },
        // )
        // .geometry_shader(Some(SpirvEntryPoint {
        // binary: TILES_GEOM,
        // entry_point: "main",
        // })),
        // ),
        // )
        // .unwrap();
        // log::info!("Created shader program: tiles");

        // let end = std::time::Instant::now();

        // log::info!("Shader creation took {}ms", (end - start).as_millis());

        Self {
            default, /* tiles */
        }
    }

    pub fn default(&self) -> &Program {
        &self.default
    }

    // pub fn tiles(&self) -> &Program {
    // &self.tiles
    // }
}
