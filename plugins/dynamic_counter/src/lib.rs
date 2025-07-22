use floem::View;
use shared::Plugin;
use shared::print_vtable_addr;
use floem::view_storage::VIEW_STORAGE;
use floem::prelude::text;

#[no_mangle]
pub extern "C" fn plugin_entry() -> Box<dyn Plugin> {
    println!("\t===== Begin plugin_entry =====");
    println!("\tFloem version: {}", env!("CARGO_PKG_VERSION"));
    print_vtable_addr(&VIEW_STORAGE, "\tVIEW_STORAGE");
    println!("\t====== End plugin_entry ======");
    Box::new(DynamicPlugin{floem_func: None})
}

struct DynamicPlugin {
    floem_func: Option<Box<dyn Fn(&str) -> Box<dyn View>>>,
}


impl Plugin for DynamicPlugin {
    fn name(&self) -> &'static str {
        "Dynamic Counter"
    }

    fn set_floem_func(&mut self, func: Box<dyn Fn(&str) -> Box<dyn View>>) {
        self.floem_func = Some(func);
    }

    fn render(&self, with_host_func: bool) -> Box<dyn View> {
        if with_host_func {
            // When `floem_func` is used, the app will launch.
            let callable_func = self.floem_func.as_ref().unwrap();
            let view = callable_func("This view is from the dynamic plugin!");
            return view;
        } else {
            // When this is used, the app will terminate without error.
            VIEW_STORAGE.with_borrow_mut(|s| println!("VIEW_STORAGE after plugin render using plugin function:\n\t{:#?}", s.view_ids));
            let view = Box::new(text("Plugin is alive?"));
            VIEW_STORAGE.with_borrow_mut(|s| println!("VIEW_STORAGE after plugin render using plugin function:\n\t{:#?}", s.view_ids));
            return view;
        }
        //view
    }
}
