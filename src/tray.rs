use crate::config::{ApiConfig, ListingType, SortType};
use crate::post::Post;
use strum::IntoEnumIterator;
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
    pub sort_items: Vec<(CheckMenuItem, SortType)>,
    pub listing_items: Vec<(CheckMenuItem, ListingType)>,
}

impl Tray {
    pub fn new(app_config: &ApiConfig) -> Self {
        let menu = Menu::new();
        let logo = MenuItem::new("🅻🅴🅼🅼🆈 ⚞ • ⚟ 🆃🆁🅰🆈", false, None);
        let quit = MenuItem::with_id(MenuActiveItemId::Quit, "Quit", true, None);
        let post_title = MenuItem::with_id(MenuActiveItemId::PostTitle, "Loading", true, None);
        let post_origin = MenuItem::new(format!("⚑ {} ", "Loading"), false, None);
        let post_counts = MenuItem::new(format!("✉ {} ⬆ {} ⬇ {}", "-", "-", "-"), false, None);

        let settings = Submenu::new("⛭ Settings", true);
        let sort_options = Submenu::new("Sort", true);
        let listing_options = Submenu::new("Listing", true);

        let sort_items: Vec<(CheckMenuItem, SortType)> = SortType::iter()
            .map(|s| {
                let s_i =
                    Self::create_check_menu_item_sort(&s.to_string(), s, app_config.sort_type);
                let _ = sort_options.append(&s_i);
                (s_i, s)
            })
            .collect();

        let listing_items: Vec<(CheckMenuItem, ListingType)> = ListingType::iter()
            .map(|l| {
                let l_i =
                    Self::create_check_menu_item_list(&l.to_string(), l, app_config.listing_type);
                let _ = listing_options.append(&l_i);
                (l_i, l)
            })
            .collect();

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
            sort_items,
            listing_items,
        }
    }

    fn create_check_menu_item_sort(
        text: &str,
        sort_type: SortType,
        config_sort_type: SortType,
    ) -> CheckMenuItem {
        CheckMenuItem::with_id(
            MenuActiveItemId::Sort(sort_type),
            text,
            true,
            sort_type == config_sort_type,
            None,
        )
    }
    fn create_check_menu_item_list(
        text: &str,
        listing_type: ListingType,
        config_listing_type: ListingType,
    ) -> CheckMenuItem {
        CheckMenuItem::with_id(
            MenuActiveItemId::Listing(listing_type),
            text,
            true,
            listing_type == config_listing_type,
            None,
        )
    }

    pub fn update(&self, post: &Post) {
        self.post_title.set_text(&post.full_title);
        self.post_origin.set_text(format!("⚑ {} ", &post.community));
        self.post_counts.set_text(format!(
            "✉ {} ⬆ {} ⬇ {}",
            &post.counts.comments, &post.counts.upvotes, &post.counts.downvotes
        ));
    }

    // we can also parse sort type from sort item id with strum ListingType::from_str(listing_item.id().as_ref()) but tuple (sort_element,SortType) looks more robust
    pub fn set_sort_checked(&self, sort_type: &SortType) {
        self.sort_items
            .iter()
            .for_each(|(sort_item, sort_option)| sort_item.set_checked(sort_option == sort_type));
    }

    pub fn set_listing_checked(&self, listing_type: &ListingType) {
        self.listing_items
            .iter()
            .for_each(|(listing_item, listing_option)| {
                listing_item.set_checked(listing_option == listing_type)
            });
    }
}
