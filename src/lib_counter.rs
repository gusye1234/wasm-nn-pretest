pub mod wasmedge_counterfunc {
    #[link(wasm_import_module = "counter_function")]
    extern "C" {
        pub fn counter_function_click();
        pub fn counter_function_get_count() -> u32;
        pub fn counter_function_forward_by(increment: u32);
    }
}

pub fn click(){
    unsafe {
        wasmedge_counterfunc::counter_function_click();
    }
}

pub fn get_count() -> u32 {
    let already_count: u32;
    unsafe {
        already_count =  wasmedge_counterfunc::counter_function_get_count();
    }
    already_count
}

pub fn forward_by(increment: u32){
    unsafe {
        wasmedge_counterfunc::counter_function_forward_by(increment);
    }
}