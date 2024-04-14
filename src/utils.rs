#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        println!("LOG - {}", format!( $( $t )* ));
    }
}
