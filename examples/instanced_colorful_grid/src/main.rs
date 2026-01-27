use vita_gl_helpers::{
    attribute::{AttributeFormat, AttributeSize, AttributeTable, AttributeType},
    attribute_table,
    buffer::{Buffer, GenDelBuffersExt},
    draw::{Elements, ElementsBufU16, Mode},
    initialise_default,
    program::link_program,
    shader::load_shader,
    swap_buffers, uniform_table,
    uniforms::UniformTable,
};

uniform_table!(MyUniformTable,
  rect_dim: Uniform2fv => "rect_dim"
);

attribute_table!(MyAttributeTable,
    pos => "pos",
    color_top => "color_top",
    color_bottom => "color_bottom"
);

pub const POSITIONS: &[[f32; 2]] = &[[-0.5, 0.5], [0.0, 0.5], [-0.5, 0.0], [0.0, 0.0]];
pub const TOP_COLORS: &[u32] = &[0xFFFF0000, 0xFF0000FF, 0xFFA526FF, 0xFFFFFFFF];
pub const BOTTOM_COLORS: &[u32] = &[0xFF0000FF, 0xFFA526FF, 0xFF00FF00, 0xFF000000];
pub const INDICES: &[u16] = &[0, 1, 3, 2];

const COLOR_FORMAT: AttributeFormat = AttributeFormat {
    size: AttributeSize::FOUR,
    type_: AttributeType::UnsignedByte,
    normalized: true,
};
const POS_FORMAT: AttributeFormat = AttributeFormat {
    size: AttributeSize::TWO,
    type_: AttributeType::Float,
    normalized: false,
};

fn main() {
    initialise_default();
    let vertex_shader = load_shader(include_str!("vert.cg"), gl::VERTEX_SHADER).expect("Oops!");
    let fragment_shader = load_shader(include_str!("frag.cg"), gl::FRAGMENT_SHADER).expect("Oops!");
    let program = link_program(vertex_shader, fragment_shader).expect("Oops!");
    let utable = MyUniformTable::with_locations_from(&program).expect("Oops!");
    let atable = MyAttributeTable::with_locations_from(&program).expect("Oops!");
    let mut buffers = [Buffer::default(); 4];
    buffers.gen_buffers();
    buffers[0].data(gl::ARRAY_BUFFER, POSITIONS, gl::STATIC_DRAW);
    buffers[1].data(gl::ARRAY_BUFFER, TOP_COLORS, gl::STATIC_DRAW);
    buffers[2].data(gl::ARRAY_BUFFER, BOTTOM_COLORS, gl::STATIC_DRAW);
    buffers[3].data(gl::ELEMENT_ARRAY_BUFFER, INDICES, gl::STATIC_DRAW);
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    }
    loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        program.use_me();
        atable.enable_all();
        buffers[0].bind_to(atable.pos, POS_FORMAT, 0, 0);
        buffers[1].bind_to(atable.color_top, COLOR_FORMAT, 0, 0);
        buffers[2].bind_to(atable.color_bottom, COLOR_FORMAT, 0, 0);
        utable.rect_dim.set([0.25, -0.5]);
        ElementsBufU16 {
            indices: buffers[3],
            len: 4,
        }
        .draw_instanced(Mode::Quads, 4);
        swap_buffers();
    }
}
