# Vulkan core function initializer for Rust

## 语言 Language

[简体中文](Readme-CN.md)|Chinglish

## Intro

This crate is for the Rust language to use Vulkan by using `vkGetInstanceProcAddr()` to get all of the supported Vulkan API functions.

All of the function that returns `VkResult` were converted to return `Result<(), VkResult>` for better error-handling.

## Usage

Typical usage is to use this crate with another crate, GLFW. By using the following function import, you can finish the `VkCore` function pointer initialization:

```
unsafe extern "C" {
	fn glfwGetInstanceProcAddress(instance: VkInstance, procname: *const i8) -> *const c_void;
}
```

The function `glfwGetInstanceProcAddress()` is a wrapper for a cross-platform call to the function `vkGetInstanceProcAddr()`.

The original GLFW crate doesn't provide a good wrapper for this function. But, as `glfw-sys` comes along with the GLFW crate, we can just import the function `glfwGetInstanceProcAddress()` directly here.

The handle `VkInstance` was defined by our `vkcore-rs` crate,  not imported from the GLFW crate. As this, we can call this function to let it work for our crate.

If you can't compile `GLFW 0.60.0`, we suggest you downgrade it to `0.59.0`

## Create the `VkCore` instance to obtain the Vulkan API function pointer addresses

To instantiate `VkCore`, use the following code:

```
use std::{
	ffi::Cstring,
	ptr::null,
};
use vkcore::*;

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
```

Then, you have all of the Vulkan functions available as `vkcore`'s members.

## Without `GLFW`

GLFW's `glfwGetInstanceProcAddress()` function implementation is to import the function `vkGetInstanceProcAddr()` from the Vulkan dynamic library from the system, then call it to get the Vulkan API function pointers.

Without GLFW, you have to import `vkGetInstanceProcAddr()` yourself; it works as our needed `get_instance_proc_address()` function. Different operating systems require you to load different dynamic libraries differently.
* On Windows: Load `vulkan-1.dll`
* On Linux: Load `libvulkan.so.1`
* On BSD: Load `libvulkan.so`
* For cocoa: Load `libvulkan.1.dylib`

As long as you can import `vkGetInstanceProcAddr()`, our `vkcore-rs` crate could work normally.
