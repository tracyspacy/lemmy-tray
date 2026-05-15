use crate::api::ApiClient;
use crate::config::{ApiConfig, ListingType, SortType};
use crate::errors::Errors;
use crate::post::Post;
use crate::tray::Tray;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};
use tray_icon::{TrayIcon, TrayIconBuilder, menu::MenuEvent};
pub enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
    RefreshTick,
}

pub struct App {
    client: ApiClient,
    tray: Tray,
    post: Post,
    tray_icon: Option<TrayIcon>,
}
impl App {
    pub fn new() -> Self {
        let tray = Tray::new();
        let client = ApiClient::new(ApiConfig::default());
        let post = match client.get_post() {
            Ok(post) => post,
            Err(e) => Post::placeholder_post(e.error_msg()),
        };
        tray.update(&post);

        Self {
            client,
            tray,
            post,
            tray_icon: None,
        }
    }

    fn init(&mut self) -> Result<(), Errors> {
        self.tray_icon = Some(
            TrayIconBuilder::new()
                .with_menu(Box::new(self.tray.menu.clone()))
                .with_tooltip(&self.post.full_title)
                .with_title(&self.post.short_title)
                .build()
                .map_err(|e| Errors::TrayIcon(e.to_string()))?,
        );
        Ok(())
    }

    fn refresh(&mut self) {
        self.post = match self.client.get_post() {
            Ok(post) => post,
            Err(e) => Post::placeholder_post(e.error_msg()),
        };
        self.tray.update(&self.post);
        if let Some(t_icon) = &self.tray_icon {
            t_icon.set_title(Some(&self.post.short_title));
            let _ = t_icon.set_tooltip(Some(&self.post.full_title));
        }
    }

    fn handle_menu_event(&mut self, m_event: MenuEvent, control_flow: &mut ControlFlow) {
        let event_id = &m_event.id;
        if event_id == self.tray.quit.id() {
            self.tray_icon.take();
            *control_flow = ControlFlow::Exit;
        }

        if event_id == self.tray.post_title.id()
            && let Some(url) = &self.post.url
        {
            let _ = open::that(url);
        }

        if event_id == self.tray.sort_hot.id() {
            self.client.api_config.sort_type = SortType::Hot;
            self.tray
                .set_sort_checked(&self.client.api_config.sort_type);
        }
        if event_id == self.tray.sort_active.id() {
            self.client.api_config.sort_type = SortType::Active;
            self.tray
                .set_sort_checked(&self.client.api_config.sort_type);
        }
        if event_id == self.tray.sort_new.id() {
            self.client.api_config.sort_type = SortType::New;
            self.tray
                .set_sort_checked(&self.client.api_config.sort_type);
        }

        if event_id == self.tray.listing_all.id() {
            self.client.api_config.listing_type = ListingType::All;
            self.tray
                .set_listing_checked(&self.client.api_config.listing_type);
        }
        if event_id == self.tray.listing_local.id() {
            self.client.api_config.listing_type = ListingType::Local;
            self.tray
                .set_listing_checked(&self.client.api_config.listing_type);
        }
    }

    pub fn run(mut self, event_loop: EventLoop<UserEvent>) {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::NewEvents(tao::event::StartCause::Init) => self
                    .init()
                    .unwrap_or_else(|_| *control_flow = ControlFlow::Exit),
                Event::UserEvent(UserEvent::MenuEvent(m_event)) => {
                    self.handle_menu_event(m_event, control_flow)
                }
                Event::UserEvent(UserEvent::RefreshTick) => self.refresh(),
                _ => {}
            }
        })
    }
}
