# Rust 的 Vulkan 核心函数初始化器

## 语言

简体中文|[Chinglish](Readme.md)

## 简介

此 crate 用于 Rust 语言，通过使用 `vkGetInstanceProcAddr()` 获取所有支持的 Vulkan API 函数来使用 Vulkan。

所有返回 `VkResult` 的函数都被转换为返回 `Result<(), VkResult>`，以便更好地处理错误。

## 用法

典型的用法是将此 crate 与另一个 crate `GLFW` 一起使用。通过以下函数导入，你可以完成 `VkCore` 函数指针的初始化：

```
unsafe extern "C" {
	fn glfwGetInstanceProcAddress(instance: VkInstance, procname: *const i8) -> *const c_void;
}
```

函数 `glfwGetInstanceProcAddress()` 是对函数 `vkGetInstanceProcAddr()` 的跨平台调用的包装器。

原始 GLFW crate 并未提供此函数的良好包装器。但是，由于 `glfw-sys` 随 `GLFW` crate 一起提供，我们可以直接在此处导入函数 `glfwGetInstanceProcAddress()`。

句柄 `VkInstance` 是由我们的 `vkcore-rs` crate 定义的，而不是从 `GLFW` crate 导入的。因此，这个函数可以为我们的 crate 干活。

如果你无法编译 `GLFW 0.60.0`，建议你将其降级至 `0.59.0`。

## 创建 `VkCore` 实例以获取 Vulkan API 函数指针地址。

要实例化 `VkCore` ，请使用以下代码：

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

这样，你就可以将所有 Vulkan 函数作为 `vkcore` 的成员函数使用。

## 如何不使用 `GLFW`

`GLFW` 的 `glfwGetInstanceProcAddress()` 函数实现是从系统中的 Vulkan 动态库导入 `vkGetInstanceProcAddr()` 函数，然后调用该函数获取 Vulkan API 函数指针。

不使用 `GLFW` 时，你必须自行导入 `vkGetInstanceProcAddr()`；它的作用相当于我们所需的 `get_instance_proc_address()` 函数。不同的操作系统需要你以不同的方式加载不同的动态库。
* 在 Windows 上：加载 `vulkan-1.dll`
* 在 Linux 上：加载 `libvulkan.so.1`
* 在 BSD 上：加载 `libvulkan.so`
* 对于 Cocoa：加载 `libvulkan.1.dylib`

只要你能够导入 `vkGetInstanceProcAddr()`，我们的 `vkcore-rs` crate 就能正常工作。
