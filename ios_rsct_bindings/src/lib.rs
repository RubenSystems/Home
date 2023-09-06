use rsct::server::Server;
use lazy_static; 


lazy_static::lazy_static! {
    pub static ref ASYNC_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    .build()
    .unwrap();
}

#[no_mangle]
pub extern "C" fn create_server(port: *const u8) -> *mut std::ffi::c_void {
    let server_box = ASYNC_RUNTIME.block_on(async move {
        Box::new(Server::new("0.0.0.0", port as &str).await)
    });

    Box::into_raw(server_box) as *mut std::ffi::c_void
    
}

pub fn listen(raw_server: *mut std::ffi::c_void) -> *mut std::os::raw::c_uchar {
    let mut server: Box<Server> = unsafe { Box::from_raw(raw_server as *mut Server) };

    ASYNC_RUNTIME.block_on(async {server.recieve_once().await }).1.as_mut_ptr()
}