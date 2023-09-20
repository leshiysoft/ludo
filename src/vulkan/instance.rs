use crate::rc_string::RCString;

pub struct ApplicationInfo
{
    pub application_name: RCString,
    pub application_version: u32,
    pub engine_name: RCString,
    pub engine_version: u32,
    pub api_version: u32,
}

pub struct InstanceCreateInfo
{
    pub flags: u32,
    pub application_info: ApplicationInfo,
    pub enabled_layer_names: Vec<RCString>,
    pub enabled_extension_names: Vec<RCString>,
}

pub struct Instance
{
    pub instance_info: InstanceCreateInfo,
    p_instance: *const libc::c_void,
}
impl Default for Instance {
    fn default() -> Self 
    { 
        let application_info = ApplicationInfo{
            application_name : RCString::from_rstr(""),
            application_version : crate::vulkan::make_version(1, 0, 0),
            engine_name : RCString::from_rstr("No Engine"),
            engine_version : crate::vulkan::make_version(1, 0, 0),
            api_version : crate::vulkan::make_api_version(0, 1, 0, 0),
        };
        Instance { 
            p_instance: std::ptr::null(),
            instance_info: InstanceCreateInfo {
                flags : 0,
                application_info : application_info,
                enabled_layer_names : Vec::new(),
                enabled_extension_names : Vec::new(),
            }
        }
    }
}
impl Instance {
    pub fn create(&mut self) -> Result<(),i32>
    {
        #[allow(non_snake_case)]
        #[repr(C)]
        pub struct VkApplicationInfo
        {
            sType : libc::c_uint,
            pNext : *const libc::c_void,
            pApplicationName : *const libc::c_char,
            applicationVersion : libc::c_uint,
            pEngineName : *const libc::c_char,
            engineVersion : libc::c_uint,
            apiVersion : libc::c_uint,
        }
        #[allow(non_snake_case)]
        #[repr(C)]
        pub struct VkInstanceCreateInfo
        {
            sType : libc::c_uint,
            pNext : *const libc::c_void,
            flags : libc::c_uint,
            pApplicationInfo : *const VkApplicationInfo,
            enabledLayerCount : libc::c_uint,
            ppEnabledLayerNames : *const *const libc::c_char,
            enabledExtensionCount : libc::c_uint,
            ppEnabledExtensionNames : *const *const libc::c_char,
        }
        extern {
            fn vkCreateInstance(
                pCreateInfo: *const VkInstanceCreateInfo, 
                pAllocator: *const libc::c_void, 
                pInstance: *mut *const libc::c_void) -> libc::c_int;
        }
        const VK_STRUCTURE_TYPE_APPLICATION_INFO: u32 = 0;
        let info = &self.instance_info;
        let application_info = VkApplicationInfo{
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: std::ptr::null(), 
            pApplicationName: info.application_info.application_name.get_cstr().as_ptr(),
            applicationVersion: info.application_info.application_version,
            pEngineName: info.application_info.engine_name.get_cstr().as_ptr(),
            engineVersion: info.application_info.engine_version,
            apiVersion: info.application_info.api_version
        };
        let mut layer_names : Vec<*const libc::c_char> = Vec::with_capacity(info.enabled_layer_names.len());
        for layer_name in &info.enabled_layer_names 
        {
            layer_names.push(layer_name.get_cstr().as_ptr());
        }
        let mut extension_names : Vec<*const libc::c_char> = Vec::with_capacity(info.enabled_extension_names.len());
        for extension_name in &info.enabled_extension_names
        {
            extension_names  .push(extension_name.get_cstr().as_ptr());
        }
        const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: u32 = 1;
        let instance_create_info = VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, 
            pNext: std::ptr::null(),
            flags: 0,
            pApplicationInfo: &application_info,
            enabledLayerCount: info.enabled_layer_names.len() as u32,
            ppEnabledLayerNames: layer_names.as_ptr(),
            enabledExtensionCount: info.enabled_extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr(),
        };
        let result = unsafe {
            vkCreateInstance(&instance_create_info, std::ptr::null(), &mut self.p_instance)
        };
        const VK_SUCCESS: i32 = 0;
        if result == VK_SUCCESS
        {
            Ok(())
        }
        else 
        {
            Err(result)
        }
    }
    pub fn destroy(&mut self)
    {
        if self.p_instance.is_null()
        {
            return;
        }
        extern {
            fn vkDestroyInstance(instance: *const libc::c_void, pAllocator: *const libc::c_void);
        }
        unsafe {
            vkDestroyInstance(self.p_instance, std::ptr::null());
        };
        self.p_instance = std::ptr::null();
        println!("Vulkan instance was destroyed.");
    }
}
impl Drop for Instance {
    fn drop(&mut self)
    {
        self.destroy();
    }
}


