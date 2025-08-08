mod vkcore;

pub use vkcore::*;

#[cfg(test)]
mod tests {
    use std::{
        ffi::{c_void, CString},
        mem::transmute,
        ptr::null,
    };
    use glfw::{Action, Context, Key, SwapInterval};
    use crate::vkcore::*;

    #[test]
    fn test() {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw.create_window(1024, 768, "GLFW Window", glfw::WindowMode::Windowed).expect("Failed to create VKFW window.");

        window.set_key_polling(true);
        window.make_current();
        glfw.set_swap_interval(SwapInterval::Adaptive);

        let get_instance_proc_address: *const c_void = unsafe {transmute(glfw::ffi::glfwGetInstanceProcAddress)};
        let get_instance_proc_address: fn(VkInstance, *const i8) -> *const c_void = unsafe {transmute(get_instance_proc_address)};

        let app_info = VkApplicationInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: CString::new("vkcore-rs test").unwrap().as_ptr() as *const _,
            applicationVersion: vk_make_version(1, 0, 0),
            pEngineName: CString::new("vkcore-rs").unwrap().as_ptr() as *const _,
            engineVersion: vk_make_version(1, 0, 0),
            apiVersion: VK_API_VERSION_1_0,
        };
        let vkcore = VkCore::new(app_info, |instance, proc_name|get_instance_proc_address(instance, CString::new(proc_name).unwrap().as_ptr() as *const _));
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
