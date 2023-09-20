use crate::rc_string::RCString;
use crate::sdl2::Instance;

pub const SDL_WINDOW_VULKAN: u32 = 0x10000000;
pub const SDL_WINDOW_SHOWN: u32 = 0x00000004;

pub struct Window
{
    title: RCString,
    p_window : *const libc::c_void,
}
impl Default for Window
{
    fn default() -> Window
    {
        Window { 
            title: RCString::from_rstr(""),
            p_window: std::ptr::null(),
        }
    }
}
impl Window
{
    pub fn create_window(title: &str, width : i32, height : i32, flags : u32) -> Result<Window, ()>
    {
        let mut window = Window::default();
        window.title = RCString::from_rstr(title);
        extern 
        {
            fn SDL_CreateWindow(
                title: *const libc::c_char, 
                x : libc::c_int, 
                y : libc::c_int, 
                w : libc::c_int, 
                h : libc::c_int, 
                flags : libc::c_uint) -> *const libc::c_void;
        }   
        const SDL_WINDOWPOS_UNDEFINED: i32 = 0x1FFF0000;
        window.p_window = unsafe { 
            SDL_CreateWindow(
                window.title.get_cstr().as_ptr(), 
                SDL_WINDOWPOS_UNDEFINED, 
                SDL_WINDOWPOS_UNDEFINED, 
                width, 
                height, 
                flags) 
        };
        if window.p_window.is_null()
        {
            Err(())
        } 
        else 
        {
            Ok(window)
        }
    }
    pub fn get_vulkan_extensions(&mut self) -> Result<Vec<RCString>, ()>
    {
        let mut count : u32 = 0;
        extern 
        {
            fn SDL_Vulkan_GetInstanceExtensions(
                window: *const libc::c_void, 
                pCount: *mut libc::c_uint, 
                pNames: *mut *const libc::c_char) -> libc::c_uint;
        }
        let result = unsafe { 
            SDL_Vulkan_GetInstanceExtensions(self.p_window, &mut count, std::ptr::null_mut()) == 1 
        };
        if !result
        {
            return Err(());
        }
        let mut names : Vec<*const libc::c_char> = Vec::new();
        names.resize(count as usize, std::ptr::null());
        let result = unsafe { 
            SDL_Vulkan_GetInstanceExtensions(self.p_window, &mut count, names.as_mut_ptr()) == 1 
        };
        if !result
        {
            return Err(());
        }
        let mut extentions : Vec<RCString> = Vec::with_capacity(count as usize);
        for p_char in names 
        {
            let c_str = unsafe { std::ffi::CStr::from_ptr(p_char) };
            extentions.push(RCString::from_cstr( c_str ));
        }
        Ok(extentions)
    }
    pub fn destroy(&mut self) -> Result<(), String>
    {
        extern 
        {
            fn SDL_DestroyWindow(window: *const libc::c_void);
        }
        unsafe { SDL_DestroyWindow(self.p_window) };
        let result = Instance::get_error();
        if !result.is_empty()
        {
            Err(result)
        }
        else 
        {
            self.p_window = std::ptr::null();
            Ok(())
        }
    }
}
impl Drop for Window {
    fn drop(&mut self)
    {
        let result = self.destroy();
        if result.is_err()
        {
            println!("SDL window drop error: {}", result.unwrap_err());
        }
    }
}