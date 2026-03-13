use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::{App, Screen, InputTarget};

pub fn draw(f: &mut ratatui::Frame, app: &App) {
    match app.screen {
        Screen::Search => draw_search(f, app),
        Screen::Results => draw_results(f, app),
        Screen::Detail => draw_detail(f, app),
        Screen::Help => draw_help(f, app),
    }
}

fn draw_search(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),  // title
        Constraint::Length(3),  // tags input
        Constraint::Length(3),  // sort
        Constraint::Length(3),  // rating
        Constraint::Min(1),    // tips
        Constraint::Length(1), // status
    ]).split(f.area());

    // title
    let title = Paragraph::new("con621 - e621 Console Client")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(title, chunks[0]);

    // tags input
    let tag_style = if matches!(app.input_target, InputTarget::Tags) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let tags = Paragraph::new(app.tag_input.as_str())
        .block(Block::default().title(" Tags (space-separated) ").borders(Borders::ALL).border_style(tag_style));
    f.render_widget(tags, chunks[1]);

    // cursor
    if matches!(app.input_target, InputTarget::Tags) {
        f.set_cursor_position((chunks[1].x + app.tag_input.len() as u16 + 1, chunks[1].y + 1));
    }

    // sort
    let sort_style = if matches!(app.input_target, InputTarget::Sort) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let sort = Paragraph::new(format!("< {} > (press any key to cycle)", app.current_sort()))
        .block(Block::default().title(" Sort ").borders(Borders::ALL).border_style(sort_style));
    f.render_widget(sort, chunks[2]);

    // rating
    let rating_style = if matches!(app.input_target, InputTarget::Rating) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let rating_label = match app.current_rating() {
        "s" => "safe",
        "q" => "questionable",
        "e" => "explicit",
        _ => "all",
    };
    let rating = Paragraph::new(format!("< {} > (press any key to cycle)", rating_label))
        .block(Block::default().title(" Rating ").borders(Borders::ALL).border_style(rating_style));
    f.render_widget(rating, chunks[3]);

    // tips
    let tips = Paragraph::new("Tab: switch fields | Enter: search | Esc: quit | ?: help")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(tips, chunks[4]);

    // status
    let status = Paragraph::new(app.status_msg.as_str())
        .style(Style::default().fg(Color::Green));
    f.render_widget(status, chunks[5]);
}

fn draw_results(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ]).split(f.area());

    let items: Vec<ListItem> = app.posts.iter().enumerate().map(|(i, post)| {
        let rating_char = match post.rating.as_str() {
            "s" => "S",
            "q" => "Q",
            "e" => "E",
            _ => "?",
        };
        let ext = post.file.ext.as_deref().unwrap_or("?");
        let dims = match (post.file.width, post.file.height) {
            (Some(w), Some(h)) => format!("{w}x{h}"),
            _ => "?".to_string(),
        };
        let size_kb = post.file.size.map(|s| s / 1024).unwrap_or(0);
        let artists = post.tags.artist.join(", ");
        let artist_str = if artists.is_empty() { "unknown".to_string() } else { artists };

        let line = format!(
            " #{:<8} [{rating_char}] {:<5} {:>10} {:>6}KB  ^{:<5} *{:<4}  {}",
            post.id, ext, dims, size_kb, post.score.total, post.fav_count, artist_str
        );

        let style = if i == app.selected {
            Style::default().bg(Color::DarkGray).fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            let fg = match post.rating.as_str() {
                "s" => Color::Green,
                "q" => Color::Yellow,
                "e" => Color::Red,
                _ => Color::White,
            };
            Style::default().fg(fg)
        };
        ListItem::new(line).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .title(format!(" Results - page {} ({} posts) ", app.page, app.posts.len()))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(list, chunks[0]);

    let help = " j/k:nav | Enter:detail | o:browser | d:download | n/p:page | q:back | ?:help ";
    let status_line = if app.status_msg.is_empty() {
        help.to_string()
    } else {
        format!("{} | {}", app.status_msg, help)
    };
    let status = Paragraph::new(status_line)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(status, chunks[1]);
}

fn draw_detail(f: &mut ratatui::Frame, app: &App) {
    let Some(post) = app.current_post() else {
        return;
    };

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ]).split(f.area());

    let rating_str = match post.rating.as_str() {
        "s" => "Safe",
        "q" => "Questionable",
        "e" => "Explicit",
        _ => "Unknown",
    };
    let dims = match (post.file.width, post.file.height) {
        (Some(w), Some(h)) => format!("{w}x{h}"),
        _ => "unknown".to_string(),
    };
    let size = post.file.size.map(|s| format_size(s)).unwrap_or_else(|| "?".to_string());
    let ext = post.file.ext.as_deref().unwrap_or("?");
    let artists = post.tags.artist.join(", ");
    let characters = post.tags.character.join(", ");
    let copyrights = post.tags.copyright.join(", ");
    let species = post.tags.species.join(", ");
    let general = post.tags.general.join(", ");
    let sources = post.sources.join("\n    ");
    let created = post.created_at.as_deref().unwrap_or("unknown");

    let mut lines = vec![
        format!("  ID:         #{}", post.id),
        format!("  Rating:     {rating_str}"),
        format!("  Score:      {} (^{} v{})", post.score.total, post.score.up, post.score.down),
        format!("  Favorites:  {}", post.fav_count),
        format!("  Type:       {ext}"),
        format!("  Size:       {dims} ({size})"),
        format!("  Created:    {created}"),
        String::new(),
        format!("  Artists:    {}", if artists.is_empty() { "none" } else { &artists }),
        format!("  Characters: {}", if characters.is_empty() { "none" } else { &characters }),
        format!("  Copyright:  {}", if copyrights.is_empty() { "none" } else { &copyrights }),
        format!("  Species:    {}", if species.is_empty() { "none" } else { &species }),
        String::new(),
        "  General Tags:".to_string(),
        format!("    {general}"),
        String::new(),
        "  Sources:".to_string(),
        format!("    {sources}"),
    ];

    if !post.description.is_empty() {
        lines.push(String::new());
        lines.push("  Description:".to_string());
        for line in post.description.lines() {
            lines.push(format!("    {line}"));
        }
    }

    // Apply scroll
    let scroll = app.detail_scroll as usize;
    let visible: Vec<Line> = lines.into_iter()
        .skip(scroll)
        .map(|l| Line::from(l))
        .collect();

    let detail = Paragraph::new(visible)
        .block(Block::default()
            .title(format!(" Post #{} ", post.id))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(detail, chunks[0]);

    let status = Paragraph::new(" j/k:scroll | h/l:prev/next | o:browser | d:download | q:back | ?:help ")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(status, chunks[1]);
}

fn draw_help(f: &mut ratatui::Frame, _app: &App) {
    let help_text = vec![
        "",
        "  con621 - e621 Console Client",
        "  =============================",
        "",
        "  Search Screen:",
        "    Tab        - Cycle between fields",
        "    Enter      - Execute search",
        "    Esc        - Quit",
        "",
        "  Results Screen:",
        "    j/k, Up/Dn - Navigate posts",
        "    Enter      - View post details",
        "    o          - Open in browser",
        "    d          - Download file",
        "    n/p        - Next/prev page",
        "    q/Esc      - Back to search",
        "",
        "  Detail Screen:",
        "    j/k        - Scroll up/down",
        "    h/l        - Previous/next post",
        "    o          - Open in browser",
        "    d          - Download file",
        "    q/Esc      - Back to results",
        "",
        "  Global:",
        "    Ctrl+C     - Force quit",
        "    ?          - Toggle this help",
        "",
        "  Tags support e621 syntax:",
        "    tag1 tag2   - AND search",
        "    ~tag        - OR",
        "    -tag        - NOT",
        "",
        "  Press Esc or ? to close this help",
    ];

    let lines: Vec<Line> = help_text.iter().map(|l| Line::from(*l)).collect();
    let help = Paragraph::new(lines)
        .block(Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(help, f.area());
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes}B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
