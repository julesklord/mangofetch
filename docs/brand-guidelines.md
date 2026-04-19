# OmniGet CLI Brand Guidelines

## Purpose

This document establishes the core brand identity, voice, and messaging for `omniget-cli`. It is intended to guide documentation, repository copy, and command-line branding while keeping a clear reference to the original OmniGet desktop project.

## Project Scope

- Primary product: `omniget-cli`
- Repository focus: CLI port and shared download core
- Original upstream: https://github.com/tonhowtf/omniget

## Product Name and Tagline

- Product name: `omniget-cli`
- Core tagline: **Command-line companion to OmniGet.**
- Supporting phrase: **Use `omniget-cli` for scripted downloads, automation, and terminal workflows.**

## Brand Promise

`omniget-cli` gives users a fast, reliable way to download media, course content, and torrents from the terminal without wrestling with GUIs.

Key promises:
- Universal platform support through intelligent backend detection
- Scriptable, automation-friendly workflows for CLI users
- Open source transparency and trust
- Consistent behavior across desktop and command-line environments

## Audience

Primary audiences:
- Developers and sysadmins who need automated download workflows
- Power users who prefer terminal-first tools
- Content creators using scriptable media capture
- Learners saving courses for offline study
- Operators managing downloads in CI or headless environments

## Voice and Tone

`omniget-cli` copy should feel:
- Practical and direct
- Confident without being aggressive
- Helpful for developers and non-developers alike
- Clear about what the CLI does and how to use it

Avoid:
- Marketing fluff or vague claims
- Overly casual slang that dilutes trust
- Technical language that is not needed for a CLI audience

## Messaging Pillars

1. **Simplicity**
   - "Run one command. Get the file."
   - Focus on quick setup, clear syntax, and predictable behavior.

2. **Automation**
   - Built for scripts, batch downloads, and headless environments.
   - Easy to integrate into CI, cron jobs, and automation pipelines.

3. **Trust**
   - Uses open source tooling and transparent dependency handling.
   - Clear messaging about what is downloaded and why.

4. **Interoperability**
   - Shares the same core engine as original OmniGet.
   - Works alongside desktop, browser, and plugin workflows.

## Brand Assets

- Mascot: Loop is a heritage asset from the original project; use sparingly in CLI documentation.
- Logo: reference `static/loop.png` only when explaining the relationship to OmniGet.
- Color tone: keep docs clean, readable, and terminal-friendly.
- Typography: use monospace for examples and command snippets.

## Visual Style Notes

- Keep the documentation layout minimal and easy to scan.
- Use terminal-style code blocks for examples.
- Avoid heavy UI imagery; focus on commands, workflows, and outputs.
- Reference Loop as a friendly brand tie-back, not the main CLI experience.

## CLI Brand Relationship

The CLI should feel like part of the OmniGet family:
- Name it `omniget-cli`
- Keep the same core brand promise: easy, reliable downloads
- Use the same voice and tagline structure with a command-line focus
- Document it as "OmniGet command line companion" rather than a separate product

Example CLI description:

> OmniGet CLI brings the same intelligent download engine to your terminal. Use `omniget-cli` for scripted downloads, batch workflows, and server automation.

## Documentation Guidelines

- Primary documentation language: English
- Keep headings short and scannable
- Use the product tagline early in landing pages and docs
- Use consistent terminology: "download", "link", "quality", "progress", "plugin"
- Avoid inconsistent product names; always use `OmniGet` for the main app and `omniget-cli` for the command-line tool

## Trademark and Legal

- The OmniGet name, logo, and Loop mascot are project trademarks.
- Open source code is licensed under GPL-3.0, but the brand assets are not automatically covered by the code license.
- Reinforce this in documentation where appropriate, especially in the README and project home page.

## Next Steps

- Define the visual style guide with brand colors and logo spacing
- Create a CLI landing page or docs section under `README.md` or `docs/`
- Add messaging guidelines for marketing copy, release notes, and social assets
- Align product copy with the CLI porting plan and documentation structure
