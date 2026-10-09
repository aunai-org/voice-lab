# voice-lab

Learn how voice measurement works, and try it live. Built around [voice-core](https://github.com/aunai-org/voice-core), the pure-Rust, offline library that measures pitch, loudness, pace, pauses and voice steadiness.

- `index.html`: guided walkthrough of every concept, with small interactive demos
- `pamphlet.html`: one-page guide for new learners (print friendly)
- `dashboard.html`: record or upload audio and see level and pitch

Open `index.html` in a browser; there is no build step. Serve over `http://localhost` (for example `python3 -m http.server`) if you want microphone access.

## Status

The dashboard's numbers come from small JavaScript stand-ins, and are labelled as such on the page. The plan is to compile voice-core to WebAssembly and have the same tiles filled by the real library, lighting up pace, pauses, jitter, shimmer and HNR as voice-core gains them. The walkthrough tags each concept as in voice-core or planned; keep those tags in sync with voice-core's README.
