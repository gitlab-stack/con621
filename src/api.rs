use serde::Deserialize;

const BASE_URL: &str = "https://e621.net";
const USER_AGENT: &str = "con621/0.1.0 (console client)";

#[derive(Debug, Deserialize, Clone)]
pub struct Post {
    pub id: u64,
    pub score: Score,
    pub fav_count: u32,
    pub rating: String,
    pub file: FileInfo,
    pub preview: PreviewInfo,
    pub tags: Tags,
    pub description: String,
    pub sources: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Score {
    pub up: i32,
    pub down: i32,
    pub total: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileInfo {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub size: Option<u64>,
    pub ext: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PreviewInfo {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tags {
    pub general: Vec<String>,
    pub species: Vec<String>,
    pub character: Vec<String>,
    pub copyright: Vec<String>,
    pub artist: Vec<String>,
    pub meta: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
}

pub fn search_posts(tags: &str, page: u32, sort: &str, rating: &str) -> Result<Vec<Post>, String> {
    let mut tag_str = tags.to_string();

    if !rating.is_empty() && rating != "all" {
        tag_str.push_str(&format!(" rating:{rating}"));
    }

    if !sort.is_empty() && sort != "default" {
        let order = match sort {
            "score" => "order:score",
            "favcount" => "order:favcount",
            "new" => "order:id_desc",
            "old" => "order:id_asc",
            _ => "",
        };
        if !order.is_empty() {
            tag_str.push_str(&format!(" {order}"));
        }
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(format!("{BASE_URL}/posts.json"))
        .query(&[
            ("tags", tag_str.trim()),
            ("page", &page.to_string()),
            ("limit", "50"),
        ])
        .send()
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let data: PostsResponse = resp.json().map_err(|e| e.to_string())?;
    Ok(data.posts)
}

pub fn download_post(post: &Post) -> Result<String, String> {
    let url = post.file.url.as_deref().ok_or("No file URL")?;
    let ext = post.file.ext.as_deref().unwrap_or("bin");

    let dl_dir = dirs::download_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Downloads")))
        .ok_or("Cannot find downloads directory")?;

    let filename = format!("e621_{}.{}", post.id, ext);
    let path = dl_dir.join(&filename);

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

    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}
