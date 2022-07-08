use glium::{
    program::{ProgramCreationInput, SpirvEntryPoint, SpirvProgram},
    Display, Program,
};

macro_rules! include_shader {
    ($name:literal) => {
        include_bytes!(concat!("../../assets/shaders/", $name))
    };
}

static DEFAULT_VERT: &[u8] = include_shader!("default.vert.spv");
static DEFAULT_FRAG: &[u8] = include_shader!("default.frag.spv");

pub struct Shaders {
    default: Program,
}

impl Shaders {
    pub fn create_all(display: &Display) -> Self {
        let default = Program::new(
            display,
            ProgramCreationInput::SpirV(SpirvProgram::from_vs_and_fs(
                SpirvEntryPoint {
                    binary: DEFAULT_VERT,
                    entry_point: "main",
                },
                SpirvEntryPoint {
                    binary: DEFAULT_FRAG,
                    entry_point: "main",
                },
            )),
        )
        .unwrap();

        Self { default }
    }
}
