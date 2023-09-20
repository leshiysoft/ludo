
extern {
    pub fn SDL_PollEvent(event : * mut libc::c_void) -> libc::c_int;
}