#!/usr/bin/env python3
"""Summarize a Claude Code execution log (the claude-code-action `execution_file` output) as Markdown.

Prints: outcome, turns, duration, cost, every tool call that was denied permission, and Claude's final
message. Used by the agent workflows so a failed or empty run explains itself on the issue/PR.
Usage: claude_run_summary.py <execution_file> [--max-chars N]
"""
from __future__ import annotations

import json
import sys
from pathlib import Path


def load_messages(path: Path) -> list[dict]:
    text = path.read_text(encoding="utf-8", errors="replace").strip()
    if not text:
        return []
    try:
        data = json.loads(text)
        return data if isinstance(data, list) else [data]
    except json.JSONDecodeError:  # JSON Lines
        out = []
        for line in text.splitlines():
            line = line.strip()
            if line:
                try:
                    out.append(json.loads(line))
                except json.JSONDecodeError:
                    pass
        return out


def last_assistant_text(messages: list[dict]) -> str:
    for m in reversed(messages):
        if m.get("type") == "assistant":
            parts = (m.get("message") or {}).get("content") or []
            texts = [p.get("text", "") for p in parts if isinstance(p, dict) and p.get("type") == "text"]
            if any(t.strip() for t in texts):
                return "\n".join(texts).strip()
    return ""


def summarize(messages: list[dict], max_chars: int) -> str:
    result = next((m for m in reversed(messages) if m.get("type") == "result"), {})
    lines = ["### Claude run summary", ""]
    if not messages:
        return "### Claude run summary\n\nThe execution log was empty: Claude Code may not have started (check authentication)."
    outcome = result.get("subtype", "unknown")
    if result.get("is_error"):
        outcome += " (error)"
    lines.append(f"- **Outcome:** {outcome}")
    if "num_turns" in result:
        lines.append(f"- **Turns:** {result['num_turns']}")
    if "duration_ms" in result:
        lines.append(f"- **Duration:** {result['duration_ms'] / 1000:.0f} s")
    if "total_cost_usd" in result:
        lines.append(f"- **Cost (API-equivalent):** ${result['total_cost_usd']:.2f}")
    denials = result.get("permission_denials") or []
    if denials:
        lines.append(f"- **Permission denials:** {len(denials)}. These tool calls were blocked by `--allowedTools`:")
        for d in denials[:15]:
            tool = d.get("tool_name", "?")
            inp = d.get("tool_input") or {}
            detail = inp.get("command") or inp.get("file_path") or inp.get("url") or json.dumps(inp)[:120]
            lines.append(f"  - `{tool}`: `{str(detail)[:160]}`")
    final = (result.get("result") or last_assistant_text(messages) or "(no final message)").strip()
    if len(final) > max_chars:
        final = final[:max_chars] + "\n…(truncated)"
    lines += ["", "**Claude's final message:**", "", final]
    return "\n".join(lines)


def main() -> None:
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    max_chars = int(sys.argv[sys.argv.index("--max-chars") + 1]) if "--max-chars" in sys.argv else 3000
    path = Path(sys.argv[1])
    if not path.exists():
        print("### Claude run summary\n\nNo execution log was produced: the Claude step did not run to completion.")
        return
    print(summarize(load_messages(path), max_chars))


if __name__ == "__main__":
    main()
