use cocoa::{
    appkit::{
        NSView, NSViewHeightSizable, NSViewWidthSizable, NSVisualEffectBlendingMode,
        NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindow,
        NSWindowOrderingMode, NSApplicationActivationOptions, NSColor, NSWindowStyleMask,
    },
    base::{nil, id},
    foundation::NSAutoreleasePool,
};
use objc::{runtime::{Object, BOOL}, *};
use raw_window_handle::HasRawWindowHandle;
use vello::peniko::Color;

pub fn set_blur(raw_window: &impl HasRawWindowHandle) {
    unsafe {
        match raw_window.raw_window_handle() {
            raw_window_handle::RawWindowHandle::AppKit(handle) => {
                // let window = handle.ns_window as *mut Object;
                let view = handle.ns_view as *mut Object;
                // let _: *mut Object = msg_send![window, setTitlebarAppearsTransparent: 0];

                // let visual_effect_class = class!(NSVisualEffectView);
                // let visual_effect: *mut Object = msg_send![visual_effect_class, new];
                // let _: () = msg_send![visual_effect, setBlendingMode: 0];
                // let _: () = msg_send![visual_effect, setMaterial: 4];
                // let _: () = msg_send![visual_effect, setState: 1];

                // let _: () = msg_send![window, setContentView: visual_effect];

                // let _: () = msg_send![view, addSubview: visual_effect positioned: 0 relativeTo: 0];

                // let ns_view = window.contentView();
                // let windwo = handle.ns_window as *mut dyn NSWindow;

                let ns_app: id = msg_send![class!(NSApplication), sharedApplication];
                let active: bool = msg_send![ns_app, isActive];
                if !active {
                    let dock_bundle_id: id = str_to_nsstring("com.apple.dock");
                    let dock_array: id = msg_send![
                        class!(NSRunningApplication),
                        runningApplicationsWithBundleIdentifier: dock_bundle_id
                    ];
                    let dock_array_len: u64 = msg_send![dock_array, count];
                    if dock_array_len == 0 {
                        panic!("Dock not running");
                    } else {
                        let dock: id = msg_send![dock_array, objectAtIndex: 0];
                        let _status: BOOL = msg_send![
                            dock,
                            activateWithOptions: NSApplicationActivationOptions::NSApplicationActivateIgnoringOtherApps
                        ];
                        let ns_running_app: id =
                            msg_send![class!(NSRunningApplication), currentApplication];
                        let () = msg_send![
                            ns_running_app,
                            activateWithOptions: NSApplicationActivationOptions::NSApplicationActivateIgnoringOtherApps
                        ];
                    }
                }



                let bounds = NSView::bounds(view);
                let blurred_view =
                    NSVisualEffectView::initWithFrame_(NSVisualEffectView::alloc(nil), bounds);
                blurred_view.autorelease();

                // NSColor::colorWithSRGBRed_green_blue_alpha_(_, r, g, b, a)

                let color = NSColor::colorWithRed_green_blue_alpha_(nil, 1.0, 0.0, 0.0, 0.0);               
                (handle.ns_window as *mut Object).setBackgroundColor_(color);

                blurred_view.setMaterial_(NSVisualEffectMaterial::HudWindow);
                blurred_view.setBlendingMode_(NSVisualEffectBlendingMode::BehindWindow);
                blurred_view.setState_(NSVisualEffectState::FollowsWindowActiveState);
                blurred_view.setAutoresizingMask_(NSViewWidthSizable | NSViewHeightSizable);

                let window = handle.ns_window as *mut Object;
                NSWindow::setTitleVisibility_(handle.ns_window as *mut Object, cocoa::appkit::NSWindowTitleVisibility::NSWindowTitleHidden);
                NSWindow::setTitlebarAppearsTransparent_(handle.ns_window as *mut Object, true);
                let style_mask = NSWindow::styleMask(window);
                NSWindow::setStyleMask_(window, style_mask | NSWindowStyleMask::NSFullSizeContentViewWindowMask);

                let _: () = msg_send![view, addSubview: blurred_view positioned: NSWindowOrderingMode::NSWindowBelow relativeTo: 0];
            }
            _ => panic!(),
        }
    }
}

fn str_to_nsstring(arg: &str) -> *mut Object {
    unsafe {
        let ns_string: id = msg_send![class!(NSString), alloc];
        let ns_string: id = msg_send![
            ns_string,
            initWithBytes: arg.as_ptr()
            length: arg.len()
            encoding: 4 as id
        ];
        ns_string
    }
}

pub fn get_color() -> Color {
    unsafe {
        let ns_color = class!(NSColor);
        let ns_color_space = class!(NSColorSpace);

        let color: *mut Object = msg_send![ns_color, windowBackgroundColor];
        let color_space: *mut Object = msg_send![ns_color_space, sRGBColorSpace];

        let color: *mut Object = msg_send![color, colorUsingColorSpace: color_space];
        let red: f64 = msg_send![color, redComponent];
        let green: f64 = msg_send![color, greenComponent];
        let blue: f64 = msg_send![color, blueComponent];

        // cacao::color::Color::MacOSWindowBackgroundColor.cg_color().

        Color::rgb(red, green, blue)
    }
}
