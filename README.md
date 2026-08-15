# Taskhomie

<img width="846" height="606" alt="Taskhomie" src="https://github.com/user-attachments/assets/b5b7de82-ec58-424f-af68-e9287a6422d6" />

**A local AI agent that controls your computer.**

Give it natural language instructions and watch it take screenshots, move your mouse, click, type, and run terminal commands. It can also drive a real browser session and talk back to you with a voice.

Built with **Tauri 2**, **React**, **TypeScript**, and **Rust**. Runs fully on your machine — no cloud backend, just your API keys.

[![Taskhomie](https://img.shields.io/badge/Taskhomie-AI%20Agent-8b5cf6?style=for-the-badge)](https://github.com/rahulcvwebsitehosting/taskhomie)
[![License](https://img.shields.io/github/license/rahulcvwebsitehosting/taskhomie?style=for-the-badge)](LICENSE)

## Demo

https://github.com/user-attachments/assets/8edd92a7-7d3e-472a-9e48-3b561f0257d6

> Used here to autonomously read and reply to tweets — purely for demonstration/research.

## What it does

- **Sees your screen** — captures screenshots and feeds them to a vision-capable model.
- **Controls your computer** — moves the cursor, clicks, types, scrolls, and runs shell commands.
- **Drives a real browser** — logs into sites with a dedicated Chrome profile and automates web tasks via the Chrome DevTools Protocol.
- **Talks** — push-to-talk voice input (Deepgram STT) and spoken replies (ElevenLabs TTS).
- **Remembers** — conversations are stored locally with SQLite and can be resumed.
- **Stays out of the way** — a tiny always-on-top pill, a floating voice orb, and a transparent border overlay during computer-control mode.

## Modes

| Mode | What it does | Use it for |
| --- | --- | --- |
| **Computer Use** | Takes over your screen. Sees what you see via screenshots and controls your cursor and keyboard directly. You step away while it works. | Tasks that span multiple apps or need full desktop access. |
| **Browser / Background** | Runs async while you do other things. Uses the Chrome DevTools Protocol for web automation and the terminal for everything else. Doesn't touch your mouse or keyboard. | Faster, more reliable web + terminal workflows. |

## Voice

Turn on voice mode to talk to the agent instead of typing:

- **Deepgram** for speech-to-text (microphone capture via `cpal`).
- **ElevenLabs** for text-to-speech, with a few preset voices plus support for a custom voice ID.
- Push-to-talk shortcuts launch the agent straight into Computer or Browser mode.

## Supported providers & models

Taskhomie works with 10 providers. Several offer free tiers, so you can run it without paying for API access. Pick a provider and model from the UI — keys are saved into a local `.env` in the app directory.

| Provider | Env var | Free tier |
| --- | --- | --- |
| Anthropic | `ANTHROPIC_API_KEY` | — |
| Nvidia NIM | `NVIDIA_API_KEY` | ✅ |
| OpenRouter | `OPENROUTER_API_KEY` | ✅ (many free models) |
| Mistral | `MISTRAL_API_KEY` | ✅ |
| Google Gemini | `GEMINI_API_KEY` | ✅ |
| Groq | `GROQ_API_KEY` | ✅ |
| Cerebras | `CEREBRAS_API_KEY` | ✅ |
| Together AI | `TOGETHER_API_KEY` | — |
| DeepSeek | `DEEPSEEK_API_KEY` | — |
| OpenCode Zen | `OPENCODE_ZEN_API_KEY` | ✅ |

Voice requires two extra keys regardless of provider:

- `DEEPGRAM_API_KEY` — speech-to-text
- `ELEVENLABS_API_KEY` — text-to-speech

## Setup

**Requirements**

- Rust & Cargo
- Node.js & npm
- API keys for at least one provider (see above)

```bash
# install dependencies
npm install

# add your API key(s) — keys live in a local .env
echo "ANTHROPIC_API_KEY=your-key-here" > .env

# run in development
npm run tauri dev

# build a production binary
npm run tauri build
```

On macOS you'll be prompted to grant **Accessibility**, **Screen Recording**, and **Microphone** permissions (System Settings → Privacy & Security). You can review and grant them anytime from the in-app Settings panel.

## Shortcuts

| Shortcut | Action |
| --- | --- |
| `⌘⇧␣` | Spotlight |
| `⌘⇧H` | Screenshot + Ask (quick prompt on the current screen) |
| `⌘⇧V` | Push-to-Talk |
| `⌃⇧C` | Voice → Computer Mode |
| `⌃⇧B` | Voice → Browser Mode |
| `⌘⇧S` | Stop Agent |
| `⌘⇧Q` | Quit |

## Settings

The in-app Settings panel lets you:

- Check **Accessibility / Screen Recording / Microphone** permission status and request them.
- Add or edit **API keys** for every provider and the voice services.
- Pick a **voice** preset or enter a custom ElevenLabs voice ID.
- Manage the dedicated **Chrome profile** used for browser automation — open it, view saved sessions, or reset it.

## Architecture

```
src/                 React + TypeScript frontend (Tauri webview)
  components/        Chat, messages, settings, voice orb UI
  hooks/useAgent.ts  Bridges the UI to the Rust backend via Tauri events
  stores/            Zustand state
  utils/             Audio (TTS/SFX) + tool formatting
src-tauri/           Rust backend
  agent.rs           Orchestrates the agent loop + streaming updates
  api.rs             Provider-agnostic chat completions
  providers.rs       10-provider model catalog
  computer.rs        Screenshot + mouse/keyboard control (xcap, enigo)
  browser.rs         Chrome automation via Chrome DevTools Protocol
  bash.rs            Local shell execution
  voice.rs           Mic capture + Deepgram STT + ElevenLabs TTS
  storage.rs         SQLite conversation history
  panels.rs          macOS panel / tray window management
```

### How it works

1. The UI sends a prompt (and, in computer mode, a screenshot) to the Rust backend.
2. The backend builds a tool-using conversation and streams it to the selected provider.
3. The model returns actions — `click`, `type`, `scroll`, `screenshot`, `bash`, or browser commands.
4. Those actions are executed locally (via `enigo`/`xcap` for the desktop, `chromiumoxide` for the browser, or a shell) and the results are fed back into the loop until the task is done.

## Stack

- **Frontend**: React 19, TypeScript, Tailwind CSS, Zustand, Framer Motion, `streamdown`
- **Backend**: Rust, Tauri 2, Tokio
- **Desktop control**: `enigo` (input), `xcap` (screenshots), `image` (processing)
- **Browser automation**: `chromiumoxide` (Chrome DevTools Protocol)
- **Voice**: `cpal` (mic), `deepgram` (STT), `elevenlabs` (TTS)
- **Storage**: `rusqlite` (SQLite)
- **Distribution**: Tauri bundling for macOS, Windows, and Linux

## Contributing

PRs welcome. Open an issue or start a discussion.

## License

[Apache License 2.0](LICENSE)
