use rsct::client::Client;
use rsct::reassembler::Reassembler;
use rsct::{allocators::basic_allocator::BasicAllocator, server::Server};
use std::ffi::CStr;
use std::os::raw::c_char;
type AllocatorType = BasicAllocator;
// use tokio::time::{sleep, Duration};

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
pub extern "C" fn create_new_reassembler() -> *mut std::ffi::c_void {
    let box_val = Box::new(Reassembler::<AllocatorType>::new(AllocatorType {}));

    Box::into_raw(box_val) as *mut std::ffi::c_void
}

#[no_mangle]
pub extern "C" fn create_server(
    port: *const u8,
    portlen: usize,
    sched: *mut std::ffi::c_void
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
    data: *const std::ffi::c_void,
    length: usize,
    capacity: usize,
    success: bool,
}

async fn recieve_whole_message(
    server: &Server,
    reassembler: &mut std::mem::ManuallyDrop<Box<Reassembler<AllocatorType>>>,
) -> (Option<Client>, Vec<u8>) {
    loop {
        let recieved = server.recieve_once().await;

        let data = match recieved {
            Ok(d) => d,
            Err(_) => continue,
        };

        match reassembler.add(data) {
            rsct::reassembler::ReassemblerResult::Complete(client, data) => return (client, data),
            rsct::reassembler::ReassemblerResult::NotComplete => continue,
        };
    }
}

#[no_mangle]
pub extern "C" fn listen_once(
    raw_server: *mut std::ffi::c_void,
    sched: *mut std::ffi::c_void,
    reassembler: *mut std::ffi::c_void,
) -> CameraData {
    let server: std::mem::ManuallyDrop<Box<Server>> =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(raw_server as *mut Server) });

    let mut reassembler: std::mem::ManuallyDrop<Box<Reassembler<AllocatorType>>> =
        std::mem::ManuallyDrop::new(unsafe {
            Box::from_raw(reassembler as *mut Reassembler<AllocatorType>)
        });
    let scheduler = std::mem::ManuallyDrop::new(unsafe {
        Box::from_raw(sched as *mut tokio::runtime::Runtime)
    });

    let res =
        scheduler.block_on(async { 
            tokio::select! {
                message = async { 
                    let message = recieve_whole_message(&server, &mut reassembler).await;
                    Some(message)
                } => message,
                // timeout = async {
                //     sleep(Duration::from_millis(timeout)).await;
                //     None   
                // } => timeout
            }

            
        });

    match res {
        Some((_, message)) => CameraData {
            data: message.as_ptr() as *const std::ffi::c_void,
            length: message.len(),
            capacity: message.capacity(),
            success: true,
        },
        None => CameraData {
            data: 0 as *mut std::ffi::c_void,
            length: 0,
            capacity: 0,
            success: false,
        }
    }
}

#[no_mangle]
pub extern "C" fn send_connection_ping(
    raw_server: *mut std::ffi::c_void,
    sched: *mut std::ffi::c_void,
    client_string: *mut std::ffi::c_char,
) {
    let server: std::mem::ManuallyDrop<Box<Server>> =
        std::mem::ManuallyDrop::new(unsafe { Box::from_raw(raw_server as *mut Server) });
    let scheduler = std::mem::ManuallyDrop::new(unsafe {
        Box::from_raw(sched as *mut tokio::runtime::Runtime)
    });
    let data = [0_u8; 1];
    let c_string: *const c_char = client_string;
    let c_str = unsafe { CStr::from_ptr(c_string) };
    let client = rsct::client::Client::from_string(c_str.to_str().unwrap().to_string());
    scheduler.block_on(async move {
        server.transmit(&data, &client).await;
    })
}

#[no_mangle]
pub extern "C" fn drop_camera_data(data: CameraData) {
    if data.success == true {
        let vec = unsafe { Vec::from_raw_parts(data.data as *mut u8, data.length, data.capacity) };
        drop(vec);
    }
}

#[no_mangle]
pub extern "C" fn drop_configuration(
    raw_server: *mut std::ffi::c_void,
    sched: *mut std::ffi::c_void,
    reassembler_p: *mut std::ffi::c_void,
) {
    let server = unsafe { Box::from_raw(raw_server as *mut Server) };
    let scheduler = unsafe { Box::from_raw(sched as *mut tokio::runtime::Runtime) };
    let reassembler = unsafe { Box::from_raw(reassembler_p as *mut Reassembler<AllocatorType>) };

    drop(server);
    drop(reassembler);
    drop(scheduler);
}
