mod instance;
pub use instance::*;
mod window;
pub use window::*;

mod sdl2_sys;

pub const SDL_QUIT: u32 = 0x100;

const SDL_EVENT_SIZE: usize  = 56;
#[allow(non_camel_case_types)]
pub struct SDL_Event
{
    buffer : [u8; SDL_EVENT_SIZE],
}
impl SDL_Event 
{
    pub fn new() -> SDL_Event
    {
        SDL_Event { buffer: [0; SDL_EVENT_SIZE] }
    }

    pub fn get_type(&self) -> u32
    {
        unsafe { *(self.buffer.as_ptr() as *const u32) }
    }
}

#[allow(non_snake_case)]
pub fn SDL_PollEvent(event : &mut SDL_Event) -> i32
{
    unsafe { sdl2_sys::SDL_PollEvent(event.buffer.as_ptr() as *mut libc::c_void) }
}