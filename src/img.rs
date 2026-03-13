use image::imageops::FilterType;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

const USER_AGENT: &str = "con621/0.1.0 (console client)";

/// Fetch an image from a URL and render it as colored half-block lines
/// that fit within the given width/height (in terminal cells).
/// Each cell = 1 char wide, 2 pixels tall (using ▀ with fg=top, bg=bottom).
pub fn fetch_and_render(url: &str, max_w: u16, max_h: u16) -> Result<Vec<Line<'static>>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| e.to_string())?;

    let bytes = client
        .get(url)
        .send()
        .map_err(|e| e.to_string())?
        .bytes()
        .map_err(|e| e.to_string())?;

    let img = image::load_from_memory(&bytes)
        .map_err(|e| e.to_string())?;

    let pixel_w = max_w as u32;
    let pixel_h = (max_h as u32) * 2; // 2 pixels per cell row

    let img = img.resize(pixel_w, pixel_h, FilterType::Triangle);
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let mut lines = Vec::new();
    let mut y = 0u32;
    while y < h {
        let mut spans = Vec::new();
        for x in 0..w {
            let top = rgba.get_pixel(x, y);
            let bot = if y + 1 < h {
                rgba.get_pixel(x, y + 1)
            } else {
                top
            };

            let fg = Color::Rgb(top[0], top[1], top[2]);
            let bg = Color::Rgb(bot[0], bot[1], bot[2]);

            spans.push(Span::styled("▀", Style::default().fg(fg).bg(bg)));
        }
        lines.push(Line::from(spans));
        y += 2;
    }

    Ok(lines)
}

/// Draw a cached image preview into a ratatui frame area
pub fn draw_preview(f: &mut ratatui::Frame, area: ratatui::layout::Rect, image_lines: &[Line<'static>]) {
    let block = Block::default()
        .title(" Preview (i to toggle) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let visible: Vec<Line> = image_lines.iter()
        .take(inner.height as usize)
        .cloned()
        .collect();
    let img_widget = Paragraph::new(visible);
    f.render_widget(img_widget, inner);
}
