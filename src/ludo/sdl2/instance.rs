use crate::rc_string::RCString;

pub struct Instance 
{
    sdl_inited: bool,
    vulkan_library_loaded: bool,
}
impl Default for Instance 
{
    fn default() -> Self
    {
        Instance 
        {
            sdl_inited: false,
            vulkan_library_loaded: false,
        }
    }
}
impl Instance
{
    pub fn get_error() -> String
    {
        let c_buf = unsafe { sdl2_sys::SDL_GetError() };
        let c_str = unsafe { std::ffi::CStr::from_ptr(c_buf) };
        c_str.to_str().unwrap().to_owned()
    }
    pub fn init(&mut self, flags: u32) -> Result<(),(i32, String)>
    {
        if self.sdl_inited
        {
            panic!("SDL2 re-initialization attempt");
        }
        let result = unsafe 
        {
            sdl2_sys::SDL_Init(flags)
        };
        if result != 0 
        {
            let error = Instance::get_error();
            Err((result, error))
        }
        else 
        {
            self.sdl_inited = true;
            Ok(())
        }
    }
    pub fn load_vulkan(&mut self, path: Option<&str>) -> Result<(), (i32, String)>
    {
        let result = if path.is_none()
        {
            unsafe { sdl2_sys::SDL_Vulkan_LoadLibrary(std::ptr::null_mut()) }
        }
        else
        {
            let c_path = RCString::from_rstr(path.unwrap()).get_cstr().as_ptr();
            unsafe { sdl2_sys::SDL_Vulkan_LoadLibrary(c_path) }
        };
        if result != 0
        {
            let error = Instance::get_error();
            Err((result, error))
        }
        else
        {
            self.vulkan_library_loaded = true;
            Ok(())
        }
    }
//     pub fn release(&mut self) -> Result<(), String>
//     {
//         let mut released = false;
//         extern 
//         {
//             pub fn SDL_Quit();
//             pub fn SDL_Vulkan_UnloadLibrary();
//         }
//         if self.vulkan_library_loaded
//         {
//             unsafe { SDL_Vulkan_UnloadLibrary() };
//             self.vulkan_library_loaded = false;
//             released = true;
//         }
//         if self.sdl_inited
//         {
//             unsafe { SDL_Quit() };
//             self.sdl_inited = false;
//             released = true;
//         }
//         if !released
//         {
//             return Ok(())
//         }
//         let error = Instance::get_error();
//         if error.is_empty()
//         {
//             Ok(())
//         }
//         else 
//         {
//             Err(error)
//         }
//     }
}
// impl Drop for Instance {
//     fn drop(&mut self)
//     {
//         let result = self.release();
//         if result.is_err()
//         {
//             println!("SDL cleaned up with errors: {}", result.unwrap_err());
//         }
//     }
// }