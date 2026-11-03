use std::cmp::Reverse;

use super::load_state::LoadState;
use crate::plex::models::{AvailableLyrics, TrackLyrics};

pub struct Lyrics {
    pub track_key: Option<String>,
    pub lyric_id: Option<String>,
    pub available: LoadState<Vec<AvailableLyrics>>,
    pub lyrics: LoadState<TrackLyrics>,
}

impl Default for Lyrics {
    fn default() -> Self {
        Self {
            track_key: None,
            lyric_id: None,
            available: LoadState::Idle,
            lyrics: LoadState::Idle,
        }
    }
}

impl Lyrics {
    pub fn set_loading_available(&mut self, track_key: &str) {
        self.track_key = Some(track_key.into());
        self.available = LoadState::Loading;
    }

    pub fn set_loading_lyrics(&mut self, lyric_id: &str) {
        self.lyric_id = Some(lyric_id.into());
        self.lyrics = LoadState::Loading;
    }

    pub fn set_available(&mut self, track_key: &str, available: Vec<AvailableLyrics>) {
        if self.track_key != Some(track_key.into()) {
            return;
        }

        self.available = LoadState::Loaded(available);
        self.lyrics = LoadState::Idle;
    }

    pub fn set_available_failed(&mut self, track_key: &str) {
        if self.track_key != Some(track_key.into()) {
            return;
        }

        self.available = LoadState::Failed;
        self.lyrics = LoadState::Failed;
    }

    pub fn set_lyrics(&mut self, lyric_id: &str, lyrics: TrackLyrics) {
        if self.lyric_id != Some(lyric_id.into()) {
            return;
        }

        self.lyrics = LoadState::Loaded(lyrics);
    }

    pub fn set_lyrics_failed(&mut self, lyric_id: &str) {
        if self.lyric_id != Some(lyric_id.into()) {
            return;
        }

        self.lyrics = LoadState::Failed;
    }

    pub fn is_available_for(&self, track_key: &str) -> bool {
        self.track_key == Some(track_key.into()) && matches!(self.available, LoadState::Loaded(_))
    }

    pub fn is_loaded_for(&self, lyrics_id: &str) -> bool {
        self.lyric_id == Some(lyrics_id.into()) && matches!(self.lyrics, LoadState::Loaded(_))
    }

    pub fn best_available(&self) -> Vec<&AvailableLyrics> {
        let LoadState::Loaded(als) = &self.available else {
            return Vec::new();
        };

        let mut ranked: Vec<_> = als.iter().collect();
        ranked.sort_by_key(|al| Reverse(score_available_lyrics(al)));
        ranked
    }

    pub fn next_lyrics_after(&self, lyric_id: Option<&str>) -> Option<String> {
        let Some(id) = lyric_id else {
            return self.best_available().first().map(|al| al.id.clone());
        };

        if self.lyric_id.as_deref() != lyric_id {
            return None;
        }

        let ranked = self.best_available();
        let index = ranked.iter().position(|al| al.id == id)?;
        ranked.get(index + 1).map(|al| al.id.clone())
    }
}

fn score_available_lyrics(lyrics: &AvailableLyrics) -> usize {
    usize::from(lyrics.format == "lrc")
}
