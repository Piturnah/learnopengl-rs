// NOTE: Not yet functional
use gl::{self, types::GLuint};
use glfw::{self, Action, Context, Key};
use std::{
    ffi::{c_void, CString},
    mem, ptr,
};

trait WindowExt {
    fn handle_window_event(&mut self, event: glfw::WindowEvent);
}

impl WindowExt for glfw::Window {
    fn handle_window_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.set_should_close(true),
            glfw::WindowEvent::FramebufferSize(width, height) => {
                assert!(gl::Viewport::is_loaded());
                unsafe { gl::Viewport(0, 0, width, height) };
            }
            _ => {}
        }
    }
}

pub fn run() {
    let vertex_shader_source = CString::new(
        "#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}",
    )
    .unwrap();
    let fragment_shader_source = CString::new(
        "#version 330 core
out vec4 FragColor
void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}",
    )
    .unwrap();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe { gl::Viewport(0, 0, 800, 600) }

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    let shader_program = unsafe {
        let vertex_shader_source_ptr: *const i8 = &vertex_shader_source as *const _ as *const i8;
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source_ptr, ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader_source_ptr: *const i8 =
            &fragment_shader_source as *const _ as *const i8;
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source_ptr, ptr::null());
        gl::CompileShader(fragment_shader);

        gl::CreateProgram()
    };

    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    let mut vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let vertices_ptr: *mut c_void = &mut vertices as *mut _ as *mut c_void;

    let vbo: GLuint = 0;
    let vbo_ptr: *mut u32 = vbo as *mut u32;
    unsafe {
        gl::GenBuffers(1, vbo_ptr);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of::<[f32; 9]>() as isize,
            vertices_ptr,
            gl::STATIC_DRAW,
        );
    }
    let vao: u32 = 0;
    let vao_ptr: *mut u32 = vao as *mut u32;
    unsafe {
        gl::GenVertexArrays(1, vao_ptr);
        gl::GenBuffers(1, vbo_ptr);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of::<[f32; 9]>() as isize,
            vertices_ptr,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<f32>() as i32,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    };

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            window.handle_window_event(event);
        }
    }
}
