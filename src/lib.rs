mod vkcore;

pub use vkcore::*;

#[cfg(test)]
mod tests {
    use std::{
        ffi::{c_void, CString},
        ptr::null,
    };
    use glfw::{Action, Context, Key, SwapInterval};
    use crate::vkcore::*;

    unsafe extern "C" {
        fn glfwGetInstanceProcAddress(instance: VkInstance, procname: *const i8) -> *const c_void;
    }

    const TEST_TIME: f64 = 10.0;

    #[test]
    fn test() {
        let test_time: Option<f64> = Some(TEST_TIME);
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw.create_window(1024, 768, "GLFW Window", glfw::WindowMode::Windowed).expect("Failed to create VKFW window.");

        window.set_key_polling(true);
        window.make_current();
        glfw.set_swap_interval(SwapInterval::Adaptive);

        let app_name = CString::new("vkcore-rs test").unwrap();
        let engine_name = CString::new("vkcore-rs").unwrap();
        let app_info = VkApplicationInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: app_name.as_ptr(),
            applicationVersion: vk_make_version(1, 0, 0),
            pEngineName: engine_name.as_ptr(),
            engineVersion: vk_make_version(1, 0, 0),
            apiVersion: VK_API_VERSION_1_0,
        };
        let vkcore = VkCore::new(app_info, |instance, proc_name|unsafe {glfwGetInstanceProcAddress(instance, CString::new(proc_name).unwrap().as_ptr())});
        dbg!(vkcore);

        let start_time = glfw.get_time();
        while !window.should_close() {
            let cur_frame_time = glfw.get_time();

            window.swap_buffers();
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    _ => {}
                }
            }
            if let Some(test_time) = test_time {
                if cur_frame_time - start_time >= test_time {
                    window.set_should_close(true)
                }
            }
        }
    }
}
