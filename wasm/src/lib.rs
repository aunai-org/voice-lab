//! Thin WebAssembly wrapper so the demo pages can call voice-core directly.
//! All measuring happens in voice-core; this file only adapts types for JS.

use voice_core::pitch::{AutocorrEstimator, PitchEstimator};
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
