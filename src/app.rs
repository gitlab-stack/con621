use crate::api::{self, Post};
use crate::img;
use ratatui::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Screen {
    Search,
    Results,
    Detail,
    Help,
}

#[derive(Clone, PartialEq)]
pub enum InputTarget {
    Tags,
    Sort,
    Rating,
}

pub struct App {
    pub screen: Screen,
    pub prev_screen: Screen,
    pub should_quit: bool,

    // search
    pub tag_input: String,
    pub input_target: InputTarget,
    pub sort_options: Vec<&'static str>,
    pub sort_idx: usize,
    pub rating_options: Vec<&'static str>,
    pub rating_idx: usize,

    // results
    pub posts: Vec<Post>,
    pub selected: usize,
    pub page: u32,
    pub detail_scroll: u16,

    // image preview
    pub show_image: bool,
    pub image_cache: Option<(u64, Vec<Line<'static>>)>, // (post_id, rendered lines)

    // status
    pub status_msg: String,
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Search,
            prev_screen: Screen::Search,
            should_quit: false,
            tag_input: String::new(),
            input_target: InputTarget::Tags,
            sort_options: vec!["default", "score", "favcount", "new", "old"],
            sort_idx: 0,
            rating_options: vec!["all", "s", "q", "e"],
            rating_idx: 0,
            posts: Vec::new(),
            selected: 0,
            page: 1,
            detail_scroll: 0,
            show_image: false,
            image_cache: None,
            status_msg: String::new(),
            loading: false,
        }
    }

    pub fn current_sort(&self) -> &str {
        self.sort_options[self.sort_idx]
    }

    pub fn current_rating(&self) -> &str {
        self.rating_options[self.rating_idx]
    }

    pub fn cycle_sort(&mut self) {
        self.sort_idx = (self.sort_idx + 1) % self.sort_options.len();
    }

    pub fn cycle_rating(&mut self) {
        self.rating_idx = (self.rating_idx + 1) % self.rating_options.len();
    }

    pub fn search(&mut self) {
        self.loading = true;
        self.status_msg = "Searching...".to_string();
        match api::search_posts(&self.tag_input, self.page, self.current_sort(), self.current_rating()) {
            Ok(posts) => {
                let count = posts.len();
                self.posts = posts;
                self.selected = 0;
                self.detail_scroll = 0;
                self.status_msg = format!("{count} results (page {})", self.page);
                if count > 0 {
                    self.screen = Screen::Results;
                } else {
                    self.status_msg = "No results found".to_string();
                }
            }
            Err(e) => {
                self.status_msg = format!("Error: {e}");
            }
        }
        self.loading = false;
    }

    pub fn next_post(&mut self) {
        if !self.posts.is_empty() && self.selected < self.posts.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn prev_post(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn next_page(&mut self) {
        self.page += 1;
        self.search();
    }

    pub fn prev_page(&mut self) {
        if self.page > 1 {
            self.page -= 1;
            self.search();
        }
    }

    pub fn current_post(&self) -> Option<&Post> {
        self.posts.get(self.selected)
    }

    pub fn open_in_browser(&mut self) {
        if let Some(post) = self.current_post() {
            let url = format!("https://e621.net/posts/{}", post.id);
            if let Err(e) = open::that(&url) {
                self.status_msg = format!("Failed to open browser: {e}");
            } else {
                self.status_msg = format!("Opened post #{}", post.id);
            }
        }
    }

    pub fn toggle_image(&mut self) {
        self.show_image = !self.show_image;
        if self.show_image {
            self.load_image_for_current();
        }
    }

    pub fn load_image_for_current(&mut self) {
        let Some(post) = self.current_post() else { return };
        // Skip if already cached for this post
        if let Some((cached_id, _)) = &self.image_cache {
            if *cached_id == post.id {
                return;
            }
        }
        let post_id = post.id;
        // Use the preview URL (small image, fast to download)
        let url = post.preview.url.as_deref()
            .or(post.file.url.as_deref());
        let Some(url) = url else {
            self.status_msg = "No preview URL available".to_string();
            return;
        };
        let url = url.to_string();
        self.status_msg = "Loading preview...".to_string();
        match img::fetch_and_render(&url, 80, 30) {
            Ok(lines) => {
                self.image_cache = Some((post_id, lines));
                self.status_msg = format!("Preview loaded for #{post_id}");
            }
            Err(e) => {
                self.status_msg = format!("Preview failed: {e}");
            }
        }
    }

    pub fn download_current(&mut self) {
        if let Some(post) = self.current_post().cloned() {
            self.status_msg = "Downloading...".to_string();
            match api::download_post(&post) {
                Ok(path) => self.status_msg = format!("Saved: {path}"),
                Err(e) => self.status_msg = format!("Download failed: {e}"),
            }
        }
    }
}
