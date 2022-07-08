macro_rules! include_shader {
    ($name:literal) => {
        include_bytes!(concat!("../../assets/shaders/", $name))
    };
}

static DEFAULT_VERT: &[u8] = include_shader!("default.vert.spv");
static DEFAULT_FRAG: &[u8] = include_shader!("default.frag.spv");
