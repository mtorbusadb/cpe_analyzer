#!/usr/bin/env python3
import argparse
import datetime as dt
import json
import pathlib
import re
import subprocess


def run_git(repo: pathlib.Path, args: list[str]) -> str:
    return subprocess.check_output(
        ["git", "-C", str(repo), *args],
        text=True,
        stderr=subprocess.DEVNULL,
    ).strip()


def safe_git(repo: pathlib.Path, args: list[str]) -> str:
    try:
        return run_git(repo, args)
    except Exception:
        return "unavailable"


def load_baseline(lock_path: pathlib.Path) -> dict:
    try:
        return json.loads(lock_path.read_text(encoding="utf-8"))
    except Exception:
        return {
            "prisme_backend": {"commit_sha": "unknown"},
            "prisme_ui": {"commit_sha": "unknown"},
            "tss": {"commit_sha": "unknown"},
        }


def referenced_prisme_files(registry_path: pathlib.Path) -> list[str]:
    text = registry_path.read_text(encoding="utf-8")
    refs = re.findall(r'"((?:prisme-backend|prisme-ui|tss)/[^"\n]+\.(?:py|go|ts|js))"', text)
    return sorted(set(refs))


def main() -> int:
    parser = argparse.ArgumentParser(description="Read-only PRISME drift align report generator")
    parser.add_argument(
        "--prisme-root",
        default="/home/mtorbus/hdd/gitrep/prisme",
        help="Path containing prisme-backend, prisme-ui, tss",
    )
    parser.add_argument(
        "--output",
        default="docs/offline-analyzer/align-drift-report.md",
        help="Output markdown report path",
    )
    parser.add_argument(
        "--generated-at",
        default=None,
        help="Override generated timestamp text for deterministic output/testing",
    )
    args = parser.parse_args()

    repo_root = pathlib.Path(__file__).resolve().parents[1]
    lock_path = repo_root / "docs/offline-analyzer/prisme-baseline.lock"
    registry_path = repo_root / "src/registry.rs"
    prisme_root = pathlib.Path(args.prisme_root)

    baseline = load_baseline(lock_path)

    repos = {
        "prisme_backend": prisme_root / "prisme-backend",
        "prisme_ui": prisme_root / "prisme-ui",
        "tss": prisme_root / "tss",
    }

    current = {}
    for key, path in repos.items():
        current[key] = {
            "branch": safe_git(path, ["rev-parse", "--abbrev-ref", "HEAD"]),
            "commit_sha": safe_git(path, ["rev-parse", "HEAD"]),
            "date": safe_git(path, ["show", "-s", "--format=%ci", "HEAD"]),
        }

    refs = referenced_prisme_files(registry_path)
    missing = [p for p in refs if not (prisme_root / p).is_file()]

    now = args.generated_at or dt.datetime.now(dt.UTC).strftime("%Y-%m-%d %H:%M:%S UTC")
    lines = [
        "# Align Drift Report",
        "",
        f"Generated: {now}",
        "Mode: read-only PRISME repo scan",
        "",
        "## Baseline vs current repository versions",
        "",
        f"Baseline source: `{lock_path.relative_to(repo_root)}`",
        "",
        "Baseline values:",
        f"- `prisme-backend`: `{baseline.get('prisme_backend', {}).get('commit_sha', 'unknown')}`",
        f"- `prisme-ui`: `{baseline.get('prisme_ui', {}).get('commit_sha', 'unknown')}`",
        f"- `tss`: `{baseline.get('tss', {}).get('commit_sha', 'unknown')}`",
        "",
        "Current values:",
    ]

    for key, label in [("prisme_backend", "prisme-backend"), ("prisme_ui", "prisme-ui"), ("tss", "tss")]:
        item = current[key]
        lines.extend(
            [
                f"- `{label}`",
                f"  - branch: `{item['branch']}`",
                f"  - commit: `{item['commit_sha']}`",
                f"  - date: `{item['date']}`",
            ]
        )

    lines.extend(
        [
            "",
            "## Evidence-path integrity scan",
            "",
            f"- total referenced PRISME files in `src/registry.rs`: `{len(refs)}`",
            f"- missing referenced files: `{len(missing)}`",
            "",
            "Missing reference list:",
        ]
    )
    if missing:
        lines.extend([f"- `{m}`" for m in missing])
    else:
        lines.append("- none")

    out = repo_root / args.output
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
