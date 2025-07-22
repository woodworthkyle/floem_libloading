use floem::View;

pub fn print_vtable_addr<T: ?Sized>(val: &T, label: &str) {
    let raw: *const T = val;
    let raw = raw as *const *const ();
    unsafe {
        let vtable_ptr = *raw.add(1);
        println!("{label} vtable: {:p}", vtable_ptr);
    }
}

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn set_floem_func(&mut self, func: Box<dyn Fn(&str) -> Box<dyn View>>);
    fn render(&self, with_host_func: bool) -> Box<dyn View>;
}
