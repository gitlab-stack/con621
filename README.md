# con621

A fast, lightweight console client for [e621](https://e621.net) built in Rust.

## Features

- Search posts with full e621 tag syntax (`tag1 tag2`, `-tag`, `~tag`)
- Sort by score, favorites, newest, or oldest
- Filter by rating (safe / questionable / explicit / all)
- Vim-style navigation (`j`/`k`/`h`/`l`)
- View detailed post info (tags, score, artists, sources, description)
- Open posts in your browser
- Download files to your Downloads folder
- Pagination support

## Building

```
cargo build --release
```

Binary outputs to `target/release/con621` (~2MB).

## Usage

```
./target/release/con621
```

### Keybindings

#### Search Screen
| Key | Action |
|-----|--------|
| `Tab` | Cycle between fields |
| `Enter` | Execute search |
| `Esc` | Quit |

#### Results Screen
| Key | Action |
|-----|--------|
| `j` / `k` / `↑` / `↓` | Navigate posts |
| `Enter` | View post details |
| `o` | Open in browser |
| `d` | Download file |
| `n` / `p` | Next / previous page |
| `q` / `Esc` | Back to search |

#### Detail Screen
| Key | Action |
|-----|--------|
| `j` / `k` | Scroll up / down |
| `h` / `l` / `←` / `→` | Previous / next post |
| `o` | Open in browser |
| `d` | Download file |
| `q` / `Esc` | Back to results |

#### Global
| Key | Action |
|-----|--------|
| `?` | Toggle help |
| `Ctrl+C` | Force quit |

## Requirements

- Rust 1.56+ (2021 edition)
- Works on Windows, Linux, and macOS — no ncurses dependency
