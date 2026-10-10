//! Thin WebAssembly wrapper so the demo pages can call voice-core directly.
//! All measuring happens in voice-core; this file only adapts types for JS.

use voice_core::pitch::{AutocorrEstimator, PitchEstimator};
use voice_core::stream::{Analyzer, FrameInfo};
use voice_core::{dsp, loudness, report};
use wasm_bindgen::prelude::*;

const ANALYSIS_HZ: u32 = 16_000;

fn err(e: voice_core::Error) -> JsError {
    JsError::new(&format!("{e:?}"))
}

/// Whole-recording summary as a JSON string (duration, loudness, median pitch, voiced share).
#[wasm_bindgen]
pub fn analyze(samples: &[f32], sample_rate: u32) -> Result<String, JsError> {
    let r = report::analyze(samples, sample_rate).map_err(err)?;
    serde_json::to_string(&r).map_err(|e| JsError::new(&e.to_string()))
}

/// Level in dBFS per 25 ms frame with a 10 ms hop (audio is resampled to 16 kHz first).
#[wasm_bindgen]
pub fn level_curve(samples: &[f32], sample_rate: u32) -> Result<Vec<f32>, JsError> {
    let mono = dsp::resample(samples, sample_rate, ANALYSIS_HZ).map_err(err)?;
    loudness::frame_rms_db(&mono, 400, 160).map_err(err)
}

/// Pitch in Hz per 10 ms frame, 0.0 where the frame is unvoiced.
#[wasm_bindgen]
pub fn pitch_track(samples: &[f32], sample_rate: u32) -> Result<Vec<f32>, JsError> {
    let track = AutocorrEstimator::default()
        .estimate(samples, sample_rate)
        .map_err(err)?;
    Ok(track.iter().map(|f| f.f0_hz.unwrap_or(0.0)).collect())
}

/// Live analysis of a microphone stream. Frames come back flattened, four numbers each:
/// time in seconds, level in dBFS, speech (1.0) or silence (0.0), pitch in Hz (0.0 when unvoiced).
#[wasm_bindgen]
pub struct LiveAnalyzer {
    inner: Option<Analyzer>,
}

fn flatten(frames: Vec<FrameInfo>) -> Vec<f32> {
    let mut out = Vec::with_capacity(frames.len() * 4);
    for f in frames {
        out.extend_from_slice(&[
            f.time_s as f32,
            f.level_dbfs,
            if f.speech { 1.0 } else { 0.0 },
            f.f0_hz.unwrap_or(0.0),
        ]);
    }
    out
}

#[wasm_bindgen]
impl LiveAnalyzer {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: u32) -> Result<LiveAnalyzer, JsError> {
        Ok(LiveAnalyzer {
            inner: Some(Analyzer::new(sample_rate).map_err(err)?),
        })
    }

    /// Feeds a chunk of any size and returns the frames that are now complete.
    pub fn push(&mut self, chunk: &[f32]) -> Vec<f32> {
        self.inner.as_mut().map(|a| flatten(a.push(chunk))).unwrap_or_default()
    }

    /// Ends the stream and returns the last frames. The analyzer is unusable afterwards.
    pub fn finish(&mut self) -> Vec<f32> {
        self.inner.take().map(|a| flatten(a.finish())).unwrap_or_default()
    }
}
