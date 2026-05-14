mod api;
mod app;
mod config;
mod post;
mod tray;

use app::{App, UserEvent};
use config::DEFAULT_REFRESH_TICK;
use tao::event_loop::EventLoopBuilder;
use tray_icon::menu::MenuEvent;

fn main() {
    let mut event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    // set a menu event handler that forwards the event and wakes up the event loop
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let proxy = event_loop.create_proxy();

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(DEFAULT_REFRESH_TICK));
            let _ = proxy.send_event(UserEvent::RefreshTick);
        }
    });
    #[cfg(target_os = "macos")]
    {
        use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
        event_loop.set_activation_policy(ActivationPolicy::Accessory);
    }
    App::new().run(event_loop);
}
