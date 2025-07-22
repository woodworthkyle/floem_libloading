use floem::{View, IntoView, views::*};
use libloading::Library;
use shared::Plugin;
use std::path::PathBuf;
use floem::view_storage::VIEW_STORAGE;
use shared::print_vtable_addr;

// true: App will launch with the plugin view.
// false: App will terminate without error.
static DEBUG_858 : bool = false;

fn main() {
    println!("\t===== Begin host main =====");
    println!("\tFloem version: {}", env!("CARGO_PKG_VERSION"));
    print_vtable_addr(&VIEW_STORAGE, "\tVIEW_STORAGE");
    println!("\t====== End host main ======");
    floem::launch(app_view);
}

fn text_wrap(txt: &str) -> Box<dyn View> {
    Box::new(text(txt))
}

fn app_view() -> impl IntoView {
    println!("Launching app_view");
    let mut children = vec![
        //Box::new(label(|| "Host GUI")) as Box<dyn floem::View>
    ];
    // Load the dynamic plugin
    println!("Attempting to unsafely load dynamic plugins");
    unsafe {
        #[cfg(all(target_os = "linux", debug_assertions))]
        let plugin_path = PathBuf::from("./target/debug/libdynamic_counter.so");
        #[cfg(all(target_os = "windows", debug_assertions))]
        //let plugin_path = PathBuf::from("./target/x86_64-pc-windows-gnu/debug/dynamic_counter.dll");
        let plugin_path = PathBuf::from("./dynamic_counter.dll");
        #[cfg(all(target_os = "linux", not(debug_assertions)))]
        let plugin_path = PathBuf::from("./target/release/libdynamic_counter.so");
        #[cfg(all(target_os = "windows", not(debug_assertions)))]
        let plugin_path = PathBuf::from("./target/x86_64-pc-windows-gnu/release/dynamic_counter.dll");


        let lib = Library::new(plugin_path).expect("Failed to load plugin");
        let constructor: libloading::Symbol<unsafe extern "C" fn() -> Box<dyn Plugin>> =
            lib.get(b"plugin_entry").unwrap();
        let mut plugin = constructor();
        let plugin_name = plugin.name();
        println!("Loaded {plugin_name}");
        plugin.set_floem_func(Box::new(text_wrap));
        VIEW_STORAGE.with_borrow_mut(|s| println!("VIEW_STORAGE prior to calling plugin.render():\n\t{:#?}", s.view_ids));
        let plugin_view = plugin.render(DEBUG_858);
        VIEW_STORAGE.with_borrow_mut(|s| println!("VIEW_STORAGE after calling plugin.render():\n\t{:#?}", s.view_ids));
        println!("Rendered {plugin_name} with id: {:?}", plugin_view.id());
        children.push(plugin_view);
    }
    //children.push(plugin_view);
    
    println!("Creating final view");
    let view = v_stack((children, ));
    view
}

/*
fn _counter_view() -> impl IntoView {
    let mut counter = RwSignal::new(0);

    h_stack((
        button("Increment").action(move || counter += 1),
        label(move || format!("Value: {counter}")),
        button("Decrement").action(move || counter -= 1),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(10))
}
*/