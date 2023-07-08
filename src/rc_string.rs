use std::ffi::{CStr, CString};

#[derive(Clone)]
pub struct RCString
{
    c_string : CString,
    r_string: String,
}

impl RCString
{
    pub fn from_rstr(str: &str) -> RCString
    {
        RCString { c_string: CString::new(str).unwrap(), r_string: str.to_owned() }
    }

    pub fn from_cstr(str: &CStr) -> RCString
    {
        RCString { c_string: str.to_owned(), r_string: str.to_str().unwrap().to_owned() }
    }

    pub fn get_rstr(&self) -> &str
    {
        &self.r_string
    }

    pub fn get_cstr(&self) -> &CStr
    {
        &self.c_string
    }

    #[allow(dead_code)]
    pub fn set_cstr(&mut self, c_string: &CStr)
    {
        self.c_string = c_string.to_owned();
        self.r_string = c_string.to_str().unwrap().to_owned();
    }

    #[allow(dead_code)]
    pub fn set_rstr(&mut self, r_string: &str)
    {
        self.r_string = r_string.to_owned();
        self.c_string = CString::new(r_string).unwrap();
    }
}