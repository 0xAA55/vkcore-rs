mod vkcore;

pub use vkcore::*;

#[cfg(test)]
mod tests {
    use glfw::{Action, Context, Key, SwapInterval};
    use crate::vkcore::*;

    #[test]
    fn test() {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create VKFW window.");

        window.set_key_polling(true);
        window.make_current();
        glfw.set_swap_interval(SwapInterval::Adaptive);

        let vkcore = VKCore::new(|proc_name|window.get_proc_address(proc_name));

        dbg!(vkcore);

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }
            let cur_frame_time = glfw.get_time();


            window.swap_buffers();
        }
    }

    #[allow(dead_code)]
    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
