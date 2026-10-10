# voice-lab

Learn how voice measurement works, and try it live. Built around [voice-core](https://github.com/aunai-org/voice-core), the pure-Rust, offline library that measures pitch, loudness, pace, pauses and voice steadiness.

- `index.html` (How it works): guided walkthrough of every concept, with small interactive demos
- `pamphlet.html` (What is it?): one-page guide for new learners
- `dashboard.html` (Try it): record or upload audio and see loudness, level and pitch measured by the real voice-core, compiled to WebAssembly
- `wasm/`: tiny wrapper crate that exposes voice-core to JavaScript

## Run locally

The walkthrough and pamphlet are plain HTML. The dashboard needs the WebAssembly build:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.100 --locked
./scripts/build-wasm.sh          # writes ./pkg
python3 -m http.server 8000      # then open http://localhost:8000
```

`voice-core` is taken from its `main` branch on GitHub. Microphone access needs `localhost` or HTTPS.

## Deploy

`.github/workflows/pages.yml` builds the WebAssembly and publishes the site to GitHub Pages on every push to `main`. In the repo settings, set Pages "Source" to "GitHub Actions". Pull requests only run the build.

## Status

The dashboard shows what voice-core measures today: duration, loudness, overall level, pitch, pace (syllables per second), pause count and longest pause, plus voice-core's own quality warnings. Pitch and pauses are compared with Praat on twenty real speech clips; pace is validated on synthetic bursts only. Jitter, shimmer and HNR are a greyed "soon" tile. Recording shows a live meter, speech/silence flag and pitch from voice-core's streaming analyzer, then the batch numbers when you stop. The walkthrough tags each concept as in voice-core or planned; keep those tags in sync with voice-core's README.
