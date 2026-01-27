use vita_gl_helpers::{
    attribute::{AttributeFormat, AttributeSize, AttributeTable, AttributeType},
    attribute_table,
    buffer::{Buffer, GenDelBuffersExt},
    draw::{Elements, ElementsBufU32, Mode},
    errors::eprintln_errors,
    initialise_default,
    program::link_program,
    shader::load_shader,
    swap_buffers,
};

attribute_table!(MyAttributeTable,
  pos => "aPos",
  color => "aColor"
);

const VERTEX_POS: &[f32; 6] = &[0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
const VERTEX_COLOR: &[u32; 3] = &[0xFF0000FFu32, 0xFF00FF00, 0xFFFF0000];

fn main() {
    initialise_default();
    let program = link_program(
        load_shader(
            "
            void main(float2 aPos,float4 aColor,float4 out gl_Position : POSITION, float4 out vColor: COLOR0) {
              gl_Position = float4(aPos,0.0,1.0);
              vColor = aColor;
            }
            ",
            gl::VERTEX_SHADER,
        )
        .unwrap(),
        load_shader(
            "
            float4 main(float4 vColor: COLOR0) {
                return vColor;
            }
            ",
            gl::FRAGMENT_SHADER,
        )
        .unwrap(),
    )
    .unwrap();
    let atable = MyAttributeTable::with_locations_from(&program).unwrap();
    let mut buffers = [Buffer::default(); 3];
    buffers.gen_buffers();

    buffers[0].data(gl::ARRAY_BUFFER, VERTEX_POS, gl::STATIC_DRAW);
    buffers[1].data(gl::ARRAY_BUFFER, VERTEX_COLOR, gl::STATIC_DRAW);
    buffers[2].data(gl::ELEMENT_ARRAY_BUFFER, &[0u32, 1, 2], gl::STATIC_DRAW);
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    }
    let pos_format = AttributeFormat {
        normalized: false,
        size: AttributeSize::TWO,
        type_: AttributeType::Float,
    };
    let color_format = AttributeFormat {
        normalized: true,
        size: AttributeSize::FOUR,
        type_: AttributeType::UnsignedByte,
    };
    loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        program.use_me(); //glUseProgram
        atable.pos.enable(); //glEnableVertexAttribArray(0)
        atable.color.enable(); //glEnableVertexAttribArray(1)
        buffers[0].bind_to(atable.pos, pos_format, 0, 0); //glBindBuffer + glVertexAttribPointer
        buffers[1].bind_to(atable.color, color_format, 0, 0); //glBindBuffer + glVertexAttribPointer
        ElementsBufU32 {
            indices: buffers[2],
            len: 3,
        }
        .draw(Mode::Triangles); //glBindBuffer(GL_ELEMENT_ARRAY_BUFFER,buffers[2]) + glDrawElements
        swap_buffers(); //vglSwapBuffers(GL_FALSE)
        eprintln_errors();
    }
}
