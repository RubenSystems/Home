use rsct::server::Server;

#[no_mangle]
pub extern "C" fn create_new_scheduler() -> *mut std::ffi::c_void {
    let box_val = Box::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap(),
    );

    Box::into_raw(box_val) as *mut std::ffi::c_void
}

#[no_mangle]
pub extern "C" fn create_server(
    port: *const u8,
    portlen: usize,
    sched: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    let portstring = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(port, portlen)).to_owned()
    };

    let scheduler = std::mem::ManuallyDrop::new(unsafe {
        Box::from_raw(sched as *mut tokio::runtime::Runtime)
    });

    println!("Creating server with port: {portstring}");
    let server_box = scheduler
        .block_on(async move { Box::new(Server::new("0.0.0.0", portstring.as_str()).await) });

    Box::into_raw(server_box) as *mut std::ffi::c_void
}


#[repr(C)]
pub struct CameraData {
    data: *mut std::ffi::c_void,
    length: usize,
    capacity: usize
}

#[no_mangle]
pub extern "C" fn listen_once(
    raw_server: *mut std::ffi::c_void,
    sched: *mut std::ffi::c_void,
) -> CameraData {
    let mut server =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(raw_server as *mut Server) });
    let scheduler = std::mem::ManuallyDrop::new(unsafe {
        Box::from_raw(sched as *mut tokio::runtime::Runtime)
    });

    let mut dat = std::mem::ManuallyDrop::new(scheduler
        .block_on(async { server.recieve_once().await })
        .1);

    CameraData { 
        data: dat.as_mut_ptr() as *mut std::ffi::c_void,
        length: dat.len(),
        capacity: dat.capacity()
    }
}

#[no_mangle]
pub extern "C" fn drop_camera_data(data: CameraData) {
    let vec = unsafe { Vec::from_raw_parts(data.data as *mut u8, data.length, data.capacity) };
    drop(vec);
}