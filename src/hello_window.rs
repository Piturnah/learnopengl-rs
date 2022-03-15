use gl;
use glfw::{self, Action, Context, Key};

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
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe { gl::Viewport(0, 0, 800, 600) }

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    gl::ClearColor::load_with(|s| window.get_proc_address(s) as *const _);
    gl::Clear::load_with(|s| window.get_proc_address(s) as *const _);
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            window.handle_window_event(event);
        }
    }
}
