use crate::config::{ListingType, SortType};
use crate::post::Post;

use tray_icon::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};

#[allow(unused)]
pub struct Tray {
    pub menu: Menu,
    pub logo: MenuItem,
    pub quit: MenuItem,
    pub post_title: MenuItem,
    pub post_origin: MenuItem,
    pub post_counts: MenuItem,
    pub sort_hot: CheckMenuItem,
    pub sort_active: CheckMenuItem,
    pub sort_new: CheckMenuItem,
    pub listing_all: CheckMenuItem,
    pub listing_local: CheckMenuItem,
}

impl Tray {
    pub fn new() -> Self {
        let menu = Menu::new();
        let logo = MenuItem::new("🅻🅴🅼🅼🆈 ⚞ • ⚟ 🆃🆁🅰🆈", false, None);
        let quit = MenuItem::new("Quit", true, None);
        let post_title = MenuItem::new("Loading", true, None);
        let post_origin = MenuItem::new(format!("⚑ {} ", "Loading"), false, None);
        let post_counts = MenuItem::new(format!("✉ {} ⬆ {} ⬇ {}", "-", "-", "-"), false, None);

        let settings = Submenu::new("⛭ Settings", true);
        let sort_options = Submenu::new("Sort", true);
        let listing_options = Submenu::new("Listing", true);

        let sort_hot = CheckMenuItem::new("Sort Hot", true, true, None);
        let sort_active = CheckMenuItem::new("Sort Active", true, false, None);
        let sort_new = CheckMenuItem::new("Sort New", true, false, None);

        let listing_all = CheckMenuItem::new("All", true, true, None);
        let listing_local = CheckMenuItem::new("Local", true, false, None);

        let _ = sort_options.append_items(&[&sort_hot, &sort_active, &sort_new]);
        let _ = listing_options.append_items(&[&listing_all, &listing_local]);

        let _ = settings.append_items(&[&sort_options, &listing_options]);

        let _ = menu.append_items(&[
            &logo,
            &PredefinedMenuItem::separator(),
            &post_title,
            &post_origin,
            &post_counts,
            &PredefinedMenuItem::separator(),
            &settings,
            &quit,
        ]);

        Self {
            menu,
            logo,
            quit,
            post_title,
            post_origin,
            post_counts,
            sort_hot,
            sort_active,
            sort_new,
            listing_all,
            listing_local,
        }
    }
    pub fn update(&self, post: &Post) {
        self.post_title.set_text(&post.full_title);
        self.post_origin.set_text(format!("⚑ {} ", &post.community));
        self.post_counts.set_text(format!(
            "✉ {} ⬆ {} ⬇ {}",
            &post.counts.comments, &post.counts.upvotes, &post.counts.downvotes
        ));
    }
    pub fn set_sort_checked(&self, sort_type: &SortType) {
        self.sort_hot
            .set_checked(matches!(sort_type, SortType::Hot));
        self.sort_active
            .set_checked(matches!(sort_type, SortType::Active));
        self.sort_new
            .set_checked(matches!(sort_type, SortType::New));
    }
    pub fn set_listing_checked(&self, listing_type: &ListingType) {
        self.listing_all
            .set_checked(matches!(listing_type, ListingType::All));
        self.listing_local
            .set_checked(matches!(listing_type, ListingType::Local));
    }
}
