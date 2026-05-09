# 🌍 Supported Platforms

MangoFetch utilizes a dual-layer extraction system to ensure maximum compatibility.

## 🥇 Native High-Performance Extractors
These platforms have dedicated Rust parsers for ultra-fast metadata retrieval:
*   **YouTube** (including Shorts and Playlists)
*   **Instagram** (Reels, Stories, Posts)
*   **TikTok**
*   **Twitter / X**
*   **Reddit**
*   **Twitch** (Clips)
*   **Pinterest**
*   **Vimeo**
*   **Bilibili**
*   **Bluesky**

---

## 🥈 Generic Extractor (yt-dlp)
For everything else, MangoFetch seamlessly wraps the industry-standard **yt-dlp**. This provides support for over **1000+ additional sites**, including:
*   Facebook
*   SoundCloud
*   DailyMotion
*   And hundreds of regional news and video portals.

---

## 🧲 Protocols
MangoFetch is not limited to web URLs. We also support:
*   **Magnet Links** (BitTorrent protocol)
*   **Direct Links** (MP4, MKV, ZIP, etc. via our internal multi-segment engine)
*   **P2P Transfers** (Direct node-to-node file sharing)
