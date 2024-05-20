use objc2_foundation::{MainThreadMarker, NSDictionary, NSString, NSURL};

fn main() {
    let thread = MainThreadMarker::new().unwrap();
    unsafe {
        let s = NSString::from_str("https://google.com");
        let url = NSURL::URLWithString(&s).unwrap();
        let application = objc2_ui_kit::UIApplication::sharedApplication(thread);
        let options = NSDictionary::<objc2_foundation::NSString>::new();
        application.openURL_options_completionHandler(&url, &options, None);
    }
}
