use crate::*;
use crate::rc_string::*;

pub struct Ludo
{
    //sdl_instance: sdl2::Instance,
    //window: sdl2::Window,
    vk_instance: vulkan::Instance,
    enable_validation_layers: bool,
    available_layers: Option<Vec<vulkan::VkLayerProperties>>,
    enabled_layers: Option<Vec<RCString>>,
    available_extensions: Option<Vec<vulkan::ExtensionProperties>>,
    enabled_extensions: Option<Vec<RCString>>,
} 

impl Default for Ludo {
    fn default() -> Self 
    { 
        Ludo{
            //sdl_instance: sdl2::Instance::default(),
            //window: Window::default(),
            vk_instance: vulkan::Instance::default(),
            enable_validation_layers: true,
            available_layers: None,
            enabled_layers: None,
            available_extensions: None,
            enabled_extensions: None,
        }
    }
}

impl Ludo {
    pub fn run(&mut self) 
    {
        self.init_window();
        self.init_vulkan();
        self.main_loop();
        self.cleanup();
    }

    fn init_window(&mut self)
    {
        println!("Starting init...");

        self.sdl_instance.init(SDL_INIT_VIDEO).unwrap();
        println!("SDL_Init done");

        self.sdl_instance.load_vulkan(None).unwrap();
        println!("SDL_Vulkan_LoadLibrary done");

        self.window = Window::create_window(
            "Rust Ludo", 
            800, 
            600, 
            SDL_WINDOW_SHOWN | SDL_WINDOW_VULKAN)
            .unwrap();
        println!("SDL_CreateWindow: done");
    }

    fn init_vulkan(&mut self)
    {
        self.create_instance();
        self.pick_physical_device();
    }

    fn get_available_extensions(&mut self) -> &Vec<vulkan::ExtensionProperties>
    {
        if self.available_extensions.is_none()
        {
            self.available_extensions = Some(vulkan::get_available_extensions(None).unwrap());
            print!("Vulkan available extentions: ");
            for extension in self.available_extensions.as_ref().unwrap()
            {
                print!("{}({}) ", extension.extension_name.get_rstr(), extension.spec_version);
            }
            print!("\n");
        }
        self.available_extensions.as_ref().unwrap()
    }

    fn get_enabled_extensions(&mut self) -> &Vec<RCString>
    {
        if self.enabled_extensions.is_none()
        {
            self.enabled_extensions = Some(self.window.get_vulkan_extensions().unwrap());
            print!("Vulkan enabled extentions: ");
            for name in self.enabled_extensions.as_ref().unwrap()
            {
                print!("{} ", name.get_rstr());
            }
            print!("\n");
        }
        self.enabled_extensions.as_ref().unwrap()
    }

    fn get_available_layers(&mut self) -> &Vec<vulkan::VkLayerProperties> 
    {
        if self.available_layers.is_none()
        {
            self.available_layers = Some(vulkan::vkEnumerateInstanceLayerProperties().unwrap());
            print!("Vulkan available layers: \n");
            for layer in self.available_layers.as_ref().unwrap()
            {
                println!("\t layer name              : {}", layer.layer_name.get_rstr());
                println!("\t spec version            : {}", vulkan::api_version_to_string(layer.spec_version));
                println!("\t implementation version  : {}", layer.implementation_version);
                println!("\t description             : {}", layer.description.get_rstr());
                println!();
            }
        }
        self.available_layers.as_ref().unwrap()
    }
    
    fn get_enabled_layer_names(&mut self) -> &Vec<RCString>
    {
        if self.enable_validation_layers
        {
            let mut enabled_layers: Vec<RCString> = Vec::new();
            enabled_layers.push(RCString::from_rstr("VK_LAYER_KHRONOS_validation"));
            self.enabled_layers = Some(enabled_layers);
        }
        self.enabled_layers.as_ref().unwrap()
    }

    fn create_instance(&mut self)
    {
        self.get_available_extensions();
        let extensions_names = self.get_enabled_extensions().clone();
        self.get_available_layers();
        let enabled_layers = self.get_enabled_layer_names().clone();
        let application_info = &mut self.vk_instance.instance_info.application_info;
        application_info.application_name = RCString::from_rstr("Rust Ludo");
        let instance_info = &mut self.vk_instance.instance_info;
        instance_info.enabled_layer_names = enabled_layers;
        instance_info.enabled_extension_names = extensions_names;
        let result = self.vk_instance.create();
        if result.is_err()
        {
            panic!("Vulkan instance creation is failed with error code: {}", result.unwrap_err());
        }
    }

    fn pick_physical_device(&mut self)
    {
        // let physical_devices = vkEnumeratePhysicalDevices(&self.vk_instance).unwrap();
        // if physical_devices.len() == 0
        // {
        //     panic!("failed to find physical devices");
        // }
        // println!("Physical device count: {}", physical_devices.len());

    }

    fn main_loop(&mut self)
    {
        println!("Starting main loop...");

        let mut running = true;
        let mut event = SDL_Event::new();
        while running
        {
            while SDL_PollEvent(&mut event) != 0
            {
                if event.get_type() == SDL_QUIT
                {
                    running = false;
                    break;
                }
            }
        }
    }

    fn cleanup(&mut self)
    {
        println!("Starting cleanup...");

        self.vk_instance.destroy();
        let result = self.window.destroy();
        if result.is_ok()
        {
            println!("SDL_DestroyWindow done");    
        }
        else 
        {
            println!("SDL_DestroyWindow errors: {}", result.unwrap_err());
        }
        let result = self.sdl_instance.release();
        if result.is_ok()
        {
            println!("SDL cleaned up without errors");    
        }
        else 
        {
            println!("SDL cleaned up with errors: {}", result.unwrap_err());
        }
    }
}