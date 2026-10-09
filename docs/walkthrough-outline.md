# voice-core walkthrough: outline

Audience: learners and developers. Each step = short idea, one interactive visual, one "predict then try" question.

1. Sound as numbers: sampling, f32 in [-1, 1], sample rate
2. Frames and hops: 25 ms windows, 10 ms hop (`dsp::frames`), resampling to 16 kHz (`dsp::resample`)  [works today]
3. Loudness: RMS and dBFS per frame, integrated LUFS and gating (`loudness::*`)  [works today]
4. Pitch (f0): voiced vs unvoiced, Hz vs semitones  [on main: pitch.rs]
5. Speech and pauses: voice activity detection, long-pause threshold  [planned]
6. Pace: syllable nuclei, articulation rate  [planned]
7. Steadiness: jitter, shimmer, HNR  [planned]
8. Reports and warnings: when not to trust a number  [report.rs on main, check fields]
9. Streaming vs batch: `Analyzer`, live feedback  [planned]
10. For developers: provenance, MIT-only deps, why Praat is only a test oracle, WASM/Android/iOS targets
