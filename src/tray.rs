use crate::config::{ListingType, SortType};
use crate::post::Post;
use tray_icon::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};

pub enum MenuActiveItemId {
    Quit,
    PostTitle,
    Sort(SortType),
    Listing(ListingType),
}

// it seems strum not perfect fot nested enums since it fills inner value with default
impl core::fmt::Display for MenuActiveItemId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Quit => write!(f, "Quit"),
            Self::PostTitle => write!(f, "PostTitle"),
            Self::Sort(sort_type) => write!(f, "{sort_type}"),
            Self::Listing(listing_type) => write!(f, "{listing_type}"),
        }
    }
}

impl std::str::FromStr for MenuActiveItemId {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Quit" => Ok(Self::Quit),
            "PostTitle" => Ok(Self::PostTitle),
            _ => {
                if let Ok(sort_type) = SortType::from_str(s) {
                    return Ok(Self::Sort(sort_type));
                }
                if let Ok(listing_type) = ListingType::from_str(s) {
                    return Ok(Self::Listing(listing_type));
                }
                Err(())
            }
        }
    }
}

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
    pub sort_top_day: CheckMenuItem,
    pub sort_top_hour: CheckMenuItem,
    pub sort_new_comments: CheckMenuItem,
    pub sort_scaled: CheckMenuItem,
    pub sort_controversial: CheckMenuItem,
    pub listing_all: CheckMenuItem,
    pub listing_local: CheckMenuItem,
}

impl Tray {
    pub fn new() -> Self {
        let menu = Menu::new();
        let logo = MenuItem::new("🅻🅴🅼🅼🆈 ⚞ • ⚟ 🆃🆁🅰🆈", false, None);
        let quit = MenuItem::with_id(MenuActiveItemId::Quit, "Quit", true, None);
        let post_title = MenuItem::with_id(MenuActiveItemId::PostTitle, "Loading", true, None);
        let post_origin = MenuItem::new(format!("⚑ {} ", "Loading"), false, None);
        let post_counts = MenuItem::new(format!("✉ {} ⬆ {} ⬇ {}", "-", "-", "-"), false, None);

        let settings = Submenu::new("⛭ Settings", true);
        let sort_options = Submenu::new("Sort", true);
        let listing_options = Submenu::new("Listing", true);

        let sort_hot = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::Hot),
            "Hot",
            true,
            false,
            None,
        );
        let sort_active = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::Active),
            "Active",
            true,
            true,
            None,
        );
        let sort_new = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::New),
            "New",
            true,
            false,
            None,
        );
        let sort_top_day = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::TopDay),
            "Top Day",
            true,
            false,
            None,
        );
        let sort_top_hour = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::TopHour),
            "Top Hour",
            true,
            false,
            None,
        );
        let sort_new_comments = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::NewComments),
            "New Comments",
            true,
            false,
            None,
        );
        let sort_scaled = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::Scaled),
            "Scaled",
            true,
            false,
            None,
        );
        let sort_controversial = CheckMenuItem::with_id(
            MenuActiveItemId::Sort(SortType::Controversial),
            "Controversial",
            true,
            false,
            None,
        );

        let listing_all = CheckMenuItem::with_id(
            MenuActiveItemId::Listing(ListingType::All),
            "All",
            true,
            true,
            None,
        );
        let listing_local = CheckMenuItem::with_id(
            MenuActiveItemId::Listing(ListingType::Local),
            "Local",
            true,
            false,
            None,
        );

        let _ = sort_options.append_items(&[
            &sort_active,
            &sort_hot,
            &sort_new,
            &sort_top_day,
            &sort_top_hour,
            &sort_new_comments,
            &sort_scaled,
            &sort_controversial,
        ]);
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
            sort_top_day,
            sort_top_hour,
            sort_new_comments,
            sort_scaled,
            sort_controversial,
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
        self.sort_top_day
            .set_checked(matches!(sort_type, SortType::TopDay));
        self.sort_top_hour
            .set_checked(matches!(sort_type, SortType::TopHour));
        self.sort_new_comments
            .set_checked(matches!(sort_type, SortType::NewComments));
        self.sort_scaled
            .set_checked(matches!(sort_type, SortType::Scaled));
        self.sort_controversial
            .set_checked(matches!(sort_type, SortType::Controversial));
    }
    pub fn set_listing_checked(&self, listing_type: &ListingType) {
        self.listing_all
            .set_checked(matches!(listing_type, ListingType::All));
        self.listing_local
            .set_checked(matches!(listing_type, ListingType::Local));
    }
}
