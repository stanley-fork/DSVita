//! Optional Chinese (CJK) font support: an on-demand download + lazy merge into
//! the imgui atlas, covering only the glyphs the game names actually use so it
//! costs nothing unless installed and stays tiny when it is.

use crate::presenter::imgui::root::{ImFontAtlas_AddFontFromMemoryTTF, ImFontConfig, ImFontConfig_ImFontConfig, ImGui};
use crate::utils::{set_thread_prio_affinity, ThreadAffinity, ThreadPriority};
use reqwest::blocking::Client;
use std::collections::BTreeSet;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::{fs, mem, thread};

const FONT_FILE: &str = "wqy-microhei.ttc";
const FONT_URL: &str = "https://gitee.com/leihtg/RL-Stock/raw/master/font/wqy-microhei.ttc";
const FONT_SIZE: usize = 5177387;

/// Where the font lives, under the cartridge root.
pub fn font_path(cartridge_path: &Path) -> PathBuf {
    cartridge_path.join("font").join(FONT_FILE)
}

pub unsafe fn load_once(path: &Path, texts: impl FnOnce() -> Vec<String>) -> bool {
    static mut APPLIED: Option<bool> = None;
    if let Some(applied) = APPLIED {
        return applied;
    }
    let applied = path.exists() && {
        merge_into_atlas(path, &texts());
        true
    };
    APPLIED = Some(applied);
    applied
}

/// Zero-terminated imgui glyph range over only the non-Latin chars in `texts`
/// (Latin comes from the base font, so code points <= 0xFF are skipped). ImWchar
/// is 16-bit, so anything beyond the BMP is dropped. `None` when empty.
fn glyph_ranges(texts: &[String]) -> Option<Vec<u16>> {
    let mut chars: BTreeSet<u16> = BTreeSet::new();
    for text in texts {
        for ch in text.chars() {
            let cp = ch as u32;
            if (0x100..=0xFFFF).contains(&cp) {
                chars.insert(cp as u16);
            }
        }
    }
    if chars.is_empty() {
        return None;
    }
    let mut ranges = Vec::with_capacity(chars.len() * 2 + 1);
    for c in chars {
        ranges.push(c);
        ranges.push(c);
    }
    ranges.push(0);
    Some(ranges)
}

unsafe fn merge_into_atlas(path: &Path, texts: &[String]) {
    let Some(ranges) = glyph_ranges(texts) else {
        return;
    };
    let Ok(data) = fs::read(path) else {
        return;
    };
    // The atlas keeps raw pointers to both the TTF data and the ranges and reads
    // them lazily when it builds on the first frame (FontDataOwnedByAtlas =
    // false), so both must outlive this call — leak them for the program.
    let data: &'static [u8] = data.leak();
    let ranges: &'static [u16] = ranges.leak();

    let mut config: ImFontConfig = mem::zeroed();
    ImFontConfig_ImFontConfig(&mut config);
    config.FontDataOwnedByAtlas = false;
    config.MergeMode = true;
    ImFontAtlas_AddFontFromMemoryTTF((*ImGui::GetIO()).Fonts, data.as_ptr() as _, data.len() as _, 22f32, &config, ranges.as_ptr() as _);
}

#[derive(Default)]
struct DownloadState {
    progress: Option<(usize, usize)>,
    done: bool,
    error: String,
}

pub struct Download {
    pub path: PathBuf,
    state: Arc<Mutex<DownloadState>>,
}

impl Download {
    pub fn new() -> Self {
        Download {
            path: PathBuf::new(),
            state: Arc::new(Mutex::new(DownloadState {
                progress: None,
                done: false,
                error: String::new(),
            })),
        }
    }

    pub fn set_file_path(&mut self, path: &Path) {
        self.path = path.to_path_buf();
        self.state.lock().unwrap().done = path.exists();
    }

    /// (done, downloading, error) snapshot for rendering.
    pub fn snapshot(&self) -> (bool, Option<(usize, usize)>, String) {
        let st = self.state.lock().unwrap();
        (st.done, st.progress, st.error.clone())
    }

    /// Downloads the font on a background thread (no-op if already downloading/done).
    pub fn start(&self) {
        {
            let mut st = self.state.lock().unwrap();
            if st.progress.is_some() || st.done {
                return;
            }
            st.progress = Some((0, 0));
            st.error.clear();
        }
        let path = self.path.clone();
        let state = Arc::clone(&self.state);
        thread::Builder::new()
            .name("cjk_font_download".to_string())
            .spawn(move || {
                set_thread_prio_affinity(ThreadPriority::Default, &[ThreadAffinity::Core1]);
                let result = download(&path, &state);
                let mut st = state.lock().unwrap();
                st.progress = None;
                match result {
                    Ok(()) => st.done = true,
                    Err(e) => st.error = e,
                }
            })
            .unwrap();
    }
}

fn download(path: &Path, state: &Arc<Mutex<DownloadState>>) -> Result<(), String> {
    let client = Client::new();
    let request = client.get(FONT_URL).header("User-Agent", concat!("DSVita/", env!("CARGO_PKG_VERSION"))).build().unwrap();
    let mut response = client.execute(request).map_err(|e| format!("Connection failed: {e}\nCheck your internet connection"))?;
    if !response.status().is_success() {
        return Err(format!("Server returned status {}", response.status().as_u16()));
    }

    let total_length = response.content_length().unwrap_or(FONT_SIZE as _) as usize;
    state.lock().unwrap().progress = Some((0, total_length));
    let mut all_bytes = Vec::new();
    let mut buf = [0; 2048];
    loop {
        match response.read(&mut buf) {
            Ok(len) => {
                if len == 0 {
                    break;
                }
                all_bytes.extend(&buf[..len]);
                state.lock().unwrap().progress = Some((all_bytes.len(), total_length));
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    // Reject HTML error/redirect pages (e.g. a Gitee hot-link block) — a real
    // font starts with a known magic. The user can place the file manually then.
    let is_font = all_bytes.len() > 4 && matches!(&all_bytes[..4], b"OTTO" | b"true" | b"ttcf" | [0x00, 0x01, 0x00, 0x00]);
    if !is_font {
        return Err("Download failed. Place the font manually (see below).".to_string());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create font dir: {e}"))?;
    }
    fs::write(path, &all_bytes).map_err(|e| format!("Failed to save font: {e}"))?;
    Ok(())
}
