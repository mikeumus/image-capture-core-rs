extern crate cocoa;
extern crate dispatch;
extern crate image_capture_core;
extern crate libc;


pub mod image_capture_core_mod {
    use cocoa::appkit::{
        NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
        NSApplicationActivationPolicyRegular, NSRunningApplication,
    };
    use cocoa::base::{id, nil, BOOL};
    use cocoa::foundation::{NSAutoreleasePool, NSString, NSArray};
    use dispatch::{Queue};
    use image_capture_core::device::{
        ICDevice, ICDeviceLocationTypeMask, 
        ICDeviceTypeMask,
    };
    use image_capture_core::camera_device::ICCameraDevice;
    use image_capture_core::camera_item::ICCameraItem;
    use image_capture_core::device_browser::ICDeviceBrowser;
    use objc::declare::ClassDecl;
    use objc::runtime::{Class, Object, Sel};
    use std::ffi::{CStr, CString};

    
    /// Convert an NSString object into a Rust String
    pub fn nsstring_decode(str: id) -> String {
        unsafe {
            let cstr: *const libc::c_char = msg_send![str, UTF8String];
            let rstr = CStr::from_ptr(cstr).to_string_lossy().into_owned();
            rstr
        }
    }
    pub fn nsstring_encode(str_enc: String) -> *mut libc::c_char {
        CString::new(str_enc).unwrap().into_raw()
    }

    // deviceDidBecomeReady
    // withCompleteContentCatalog
    // https://developer.apple.com/documentation/imagecapturecore/iccameradevicedelegate/1508008-devicedidbecomeready
    extern "C" fn device_did_become_ready(
        _: &Object,
        _: Sel,
        _: id,
    ) {
        println!("  🤩📷 device_did_become_ready");
    }
    extern "C" fn device_did_open_session_with_error(
        obj_obj: &Object,
        sel_sel: Sel,
        device: id,
        error: id,
    ) {
        // let camera_items: Vec<ICCameraItem> = vec![];
        println!("  📸📸 device_did_open_session_with_error obj: {:?}", obj_obj);
        println!("  📸📸 device_did_open_session_with_error sel: {:?}", sel_sel);
        println!("  📸📸 device_did_open_session_with_error Error: {:?}", unsafe { error.as_ref() });
        println!("  📸📸 device_did_open_session_with_error device.delegate: {:?}", unsafe { ICDevice::delegate(device).as_ref().unwrap() });

        unsafe {
            let cam = device.mediaFiles().as_ref().unwrap();
            let cam_class = cam.class();
            let super_cls = cam_class.superclass().unwrap();

            for method in obj_obj.class().adopted_protocols().iter() {
                println!("  ❄️📸 obj_obj protocol: {}", method.name());
            }
            for method in cam_class.adopted_protocols().iter() {
                println!("  ❄️📸 class protocol: {}", method.name());
            }

            for ivar in cam_class.instance_variables().iter() {
                println!("  🌞📸 class ivar: {}", ivar.name());
            }
            for ivar in cam_class.instance_methods().iter() {
                println!("  📛📷 class method: {:?}", ivar.name());
            }
        }
        println!("  🍄📷 device.open: {:?}", unsafe { device.hasOpenSession() });
    }
    extern "C" fn device_did_close_session_with_error(
        _: &Object,
        _: Sel,
        device: id,
        _: id,
    ) {
        println!("  📸📸 device_did_close_session_with_error");
    }
    extern "C" fn device_did_remove_device(
        _: &Object,
        _: Sel,
        _: id,
    ) {
        println!("  📸📸 device_did_remove");
    }

    fn get_device_delegate() -> ClassDecl {
        unsafe {
            let mut decl = ClassDecl::new(
                "CameraDeviceDelegate", 
                objc::runtime::Class::get("BrowserDelegate").unwrap()
            ).unwrap();

            println!("  📸 add_method didOpenSessionWithError");
            decl.add_method(
                sel!(device:didOpenSessionWithError:),
                device_did_open_session_with_error as extern "C" fn(&Object, Sel, id, id),
            );
            println!("  📸 add_method didCloseSessionWithError");
            decl.add_method(
                sel!(device:didCloseSessionWithError:),
                device_did_close_session_with_error as extern "C" fn(&Object, Sel, id, id),
            );
            println!("  📸 add_method didRemoveDevice");
            decl.add_method(
                sel!(didRemoveDevice:),
                device_did_remove_device as extern "C" fn(&Object, Sel, id),
            );
            println!("  📸 add_method withCompleteContentCatalog");
            decl.add_method(
                sel!(withCompleteContentCatalog:),
                device_did_become_ready as extern "C" fn(&Object, Sel, id),
            );
            println!("  📸 add_method end");
            decl
        }
    }

    pub fn init(queue: Queue) {
        println!("  🙂 image_capture_core_mod init!");
        unsafe {
            queue.clone().barrier_async(move || {
                println!("  🙂 image_capture_core_mod exec_async");
                let main = Queue::main();
                
                main.barrier_async(|| {
                    println!("  🙂 image_capture_core_mod exec_sync");
                    let _pool = NSAutoreleasePool::new(nil);                   

                    // Create the device browser delegate
                    let superclass = class!(NSObject);
                    let mut decl = ClassDecl::new("BrowserDelegate", superclass).unwrap();

                    extern "C" fn device_browser_did_add_device(
                        _: &Object,
                        _: Sel,
                        cam_id: id,
                        device_self: id,
                        _more_coming: BOOL,
                    ) {
                        let device_name = unsafe { ICDevice::name(device_self) };
                        let device_type = unsafe { ICDevice::type_(device_self) };
                        let name = nsstring_decode( device_name );

                        unsafe {
                            let device_delegate = get_device_delegate();
                            let delegate_class = device_delegate.register();
                            let delegate_object = msg_send![delegate_class, new]
                            let camera_device: Box<ICCameraDevice> = Box::new(device_self.clone());

                            ICDevice::setDelegate(device_self, delegate_object);
                            // ICCameraDevice::setDelegate(camera_device, delegate_object);                                

                            println!("  📸📷 class_copyMethodList ICDeviceDelegate: {:?}", device_self.as_ref().unwrap());
                            if device_self.hasOpenSession() == 0 {
                                device_self.requestOpenSession();
                                ICDevice::requestOpenSession(device_self);
                                println!("  📸📸 image_capture_core_mod requestOpen");
                            }
                            println!("  📸📸 image_capture_core_mod hasOpenSession: {:?}", device_self.hasOpenSession());
                        }

                        println!("  📸📸 image_capture_core_mod _more_coming: {}", _more_coming);
                        println!("  📸📸 image_capture_core_mod device_self name: {}", name);
                        println!("  📸📸 image_capture_core_mod device_self type: {:?}", device_type);
                        
                    }

                    extern "C" fn device_browser_did_remove_device(
                        _: &Object,
                        _: Sel,
                        _: id,
                        device: id,
                        _more_going: BOOL,
                    ) {
                        let name = nsstring_decode(unsafe { ICDevice::name(device) });
                        println!("  📸📸 image_capture_core_mod Device removed: '{}'", name);
                    }

                    decl.add_method(
                        sel!(deviceBrowser:didAddDevice:moreComing:),
                        device_browser_did_add_device as extern "C" fn(&Object, Sel, id, id, BOOL),
                    );
                    decl.add_method(
                        sel!(deviceBrowser:didRemoveDevice:moreGoing:),
                        device_browser_did_remove_device as extern "C" fn(&Object, Sel, id, id, BOOL),
                    );

                    let delegate_class = decl.register();
                    let delegate_object = msg_send![delegate_class, new];

                    // Create the device browser
                    let browser = ICDeviceBrowser::new(nil).autorelease();
                    ICDeviceBrowser::setDelegate(browser, delegate_object);
                    println!("  🥲📷 ICDeviceBrowser.delegate: {:?}", ICDeviceBrowser::delegate(browser));
                    let types_mask 
                        ICDeviceTypeMask::ICDeviceTypeMaskCamera;
                    let locations_mask = ICDeviceLocationTypeMask::ICDeviceLocationTypeMaskLocal
                        | ICDeviceLocationTypeMask::ICDeviceLocationTypeMaskRemote;
                    browser.setBrowsedDeviceTypeMask(types_mask.bits() | locations_mask.bits());
                    browser.start();


                    // 🚨 ERROR READ: https://developer.apple.com/forums/thread/113268
                    // ✅❔ SOLUTION:
                    // - https://github.com/SSheldon/rust-dispatch
                    // - https://github.com/SSheldon/rust-dispatch/blob/master/examples/main.rs
                    // Run the application
                    let app = NSApp();
                    println!("  📱 app id: {:?}", app);
                    println!("  📱 app run browser.isBrowsing: {:?}", browser.isBrowsing());
                    let app = NSApp();
                    app.activateIgnoringOtherApps_(1);
                    app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
                    let current_app = NSRunningApplication::currentApplication(nil);
                    app.run();
                    app.finishLaunching();
                    browser.stop();
                    println!("  📱 app stop browser.isBrowsing: {:?}", browser.isBrowsing());
                    app.stop_(app);
                });
                drop(main);
            });
        }
    }
}