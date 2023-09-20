mod instance;
pub use instance::*;
mod extensions;
pub use extensions::*;
use crate::rc_string::RCString;


const VK_SUCCESS: i32 = 0;

#[derive(Clone)]
pub struct VkPhysicalDevice 
{
    p_physical_device : *const libc::c_void,
}
impl Default for VkPhysicalDevice {
    fn default() -> Self 
    { 
        VkPhysicalDevice { 
            p_physical_device: std::ptr::null(),
        }
    }
}

pub fn make_version(major: u32, minor: u32, patch: u32) -> u32
{
    (major << 22) | (minor << 12) | (patch)
}

pub fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32
{
    (variant << 29) | (major << 22) | (minor << 12) | (patch)
}

#[allow(dead_code)]
pub fn api_version_to_string(version: u32) -> String
{
    let variant = version >> 29;
    let major = (version >> 22) & 0x7F;
    let minor = (version >> 12) & 0x3FF;
    let patch = version & 0xFFF;
    return variant.to_string() + "." +
        major.to_string().as_ref() + "." +
        minor.to_string().as_ref() + "." +
        patch.to_string().as_ref();
}

pub struct VkLayerProperties
{
    pub layer_name : RCString,
    pub spec_version : u32,
    pub implementation_version : u32,
    pub description : RCString,
}

#[allow(non_snake_case)]
pub fn vkEnumerateInstanceLayerProperties() -> Result<Vec<VkLayerProperties>, i32>
{
    const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
    const VK_MAX_DESCRIPTION_SIZE: usize = 256;
    #[allow(non_snake_case)]
    #[derive(Clone)]
    #[repr(C)]
    struct sys_VkLayerProperties
    {
        layerName : [libc::c_char; VK_MAX_EXTENSION_NAME_SIZE],
        specVersion : libc::c_uint,
        implementationVersion : libc::c_uint,
        description : [libc::c_char; VK_MAX_DESCRIPTION_SIZE],
    }
    impl sys_VkLayerProperties
    {
        fn new() -> sys_VkLayerProperties
        {
            sys_VkLayerProperties { 
                layerName: [0; VK_MAX_EXTENSION_NAME_SIZE], 
                specVersion: 0,
                implementationVersion: 0,
                description: [0; VK_MAX_DESCRIPTION_SIZE]
            }
        }
        fn to_safe(&self) -> VkLayerProperties
        {
            let c_layer_name = unsafe {
                std::ffi::CStr::from_ptr(self.layerName.as_ptr())
            };
            let layer_name = RCString::from_cstr(c_layer_name);
            let c_desription = unsafe {
                std::ffi::CStr::from_ptr(self.description.as_ptr())
            };
            let description = RCString::from_cstr(c_desription);
            VkLayerProperties { 
                layer_name, 
                spec_version: self.specVersion, 
                implementation_version: self.implementationVersion, 
                description }
        }
    }
    extern {
        fn vkEnumerateInstanceLayerProperties(
            pPropertyCount: *mut libc::c_uint, 
            pProperties: *mut sys_VkLayerProperties) -> libc::c_int;
    }
    let mut property_count: u32 = 0;
    let result = unsafe {
        vkEnumerateInstanceLayerProperties(
            &mut property_count, std::ptr::null_mut())
    };
    if result != VK_SUCCESS
    {
        return Err(result);
    }
    let mut layer_properties_vector : Vec<sys_VkLayerProperties> = Vec::new();
    layer_properties_vector.resize(property_count as usize, sys_VkLayerProperties::new());
    let result = unsafe {
        vkEnumerateInstanceLayerProperties(
            &mut property_count, layer_properties_vector.as_mut_ptr())
    };
    if result != VK_SUCCESS
    {
        return Err(result);
    }
    let mut result : Vec<VkLayerProperties> = Vec::new();
    for layer_properties in layer_properties_vector
    {
        result.push(layer_properties.to_safe());
    }
    Ok(result)
}

// #[allow(non_snake_case)]
// pub fn vkEnumeratePhysicalDevices(instance: &VkInstance) -> Result<Vec<VkPhysicalDevice>, i32>
// {
//     extern {
//         fn vkEnumeratePhysicalDevices(
//             instance: *const libc::c_void, 
//             pPhysicalDeviceCount: *mut libc::c_uint,
//             pPhysicalDevices: *mut *const libc::c_void) -> libc::c_int;
//     }
//     let mut device_count: u32 = 0;
//     let result = unsafe {
//         vkEnumeratePhysicalDevices(
//             instance.p_instance, 
//             &mut device_count, 
//             std::ptr::null_mut())
//     };
//     if result != VK_SUCCESS
//     {
//         return Err(result);
//     }
//     if device_count == 0
//     {
//         return Ok(Vec::new());
//     }
//     let mut phisical_devices: Vec<VkPhysicalDevice> = Vec::new();
//     phisical_devices.resize(device_count as usize, VkPhysicalDevice::default());
//     let result = unsafe {
//         vkEnumeratePhysicalDevices(
//             instance.p_instance, 
//             &mut device_count, 
//             &mut phisical_devices[0].p_physical_device)
//     };
//     if result != VK_SUCCESS
//     {
//         return Err(result);
//     }
//     Ok(phisical_devices)
// }