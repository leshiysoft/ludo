use crate::rc_string::RCString;

pub struct ExtensionProperties
{
    pub extension_name: RCString,
    pub spec_version: u32,
}

pub fn get_available_extensions(layer_name: Option<RCString>) -> Result<Vec<ExtensionProperties>, i32>
{
    const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
    #[allow(non_snake_case)]
    #[derive(Clone)]
    #[repr(C)]
    struct sys_VkExtensionProperties
    {
        extensionName: [libc::c_char; VK_MAX_EXTENSION_NAME_SIZE],
        specVersion: libc::c_uint,
    }
    impl sys_VkExtensionProperties
    {
        fn new() -> sys_VkExtensionProperties
        {
            sys_VkExtensionProperties { 
                extensionName: [0; VK_MAX_EXTENSION_NAME_SIZE], 
                specVersion: 0 
            }
        }
        fn to_safe(&self) -> ExtensionProperties
        {
            let c_extension_name = unsafe {
                std::ffi::CStr::from_ptr(self.extensionName.as_ptr())
            };
            let extension_name = RCString::from_cstr(c_extension_name);
            ExtensionProperties { 
                extension_name, 
                spec_version: self.specVersion
            }
        }
    }
    extern {
        fn vkEnumerateInstanceExtensionProperties(
            pLayerName: *const libc::c_char,
            pPropertyCount: *mut libc::c_uint, 
            pProperties: *mut sys_VkExtensionProperties) -> libc::c_int;
    }
    let c_layer_name = match layer_name {
        Some(name) => name.get_cstr().as_ptr(),
        None => std::ptr::null(),
    };
    let mut property_count : u32 = 0;
    let result = unsafe {
        vkEnumerateInstanceExtensionProperties(
            c_layer_name, 
            &mut property_count, 
            std::ptr::null_mut()) 
    };
    const VK_SUCCESS: i32 = 0;
    if result != VK_SUCCESS
    {
        return Err(result);
    }
    let mut extension_properties_vector : Vec<sys_VkExtensionProperties> = Vec::new();
    extension_properties_vector.resize(property_count as usize, sys_VkExtensionProperties::new());
    let result = unsafe {
        vkEnumerateInstanceExtensionProperties(
            c_layer_name,
            &mut property_count, 
            extension_properties_vector.as_mut_ptr())
    };
    if result != VK_SUCCESS
    {
        return Err(result);
    }
    let mut result : Vec<ExtensionProperties> = Vec::new();
    for extension_properties in extension_properties_vector
    {
        result.push(extension_properties.to_safe());
    }
    Ok(result)
}