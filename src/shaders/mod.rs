use glium::{
    program::{ProgramCreationInput, SpirvEntryPoint, SpirvProgram},
    Display, Program,
};

macro_rules! include_shader {
    ($name:literal) => {
        include_bytes!(concat!("../../assets/shaders/", $name))
    };
}

static TILES_VERT: &[u8] = include_shader!("tiles.vert.spv");
static TILES_FRAG: &[u8] = include_shader!("tiles.frag.spv");
static TILES_GEOM: &[u8] = include_shader!("tiles.geom.spv");

pub struct Shaders {
    tiles: Program,
}

impl Shaders {
    pub fn create_all(display: &Display) -> Self {
        let tiles = Program::new(
            display,
            ProgramCreationInput::SpirV(
                SpirvProgram::from_vs_and_fs(
                    SpirvEntryPoint {
                        binary: TILES_VERT,
                        entry_point: "main",
                    },
                    SpirvEntryPoint {
                        binary: TILES_FRAG,
                        entry_point: "main",
                    },
                )
                .geometry_shader(Some(SpirvEntryPoint {
                    binary: TILES_GEOM,
                    entry_point: "main",
                })),
            ),
        )
        .unwrap();

        log::info!("Created shader program: tiles");
        Self { tiles }
    }

    pub fn tiles(&self) -> &Program {
        &self.tiles
    }
}
