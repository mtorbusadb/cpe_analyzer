#!/usr/bin/env bash
set -euo pipefail

required_files=(
  "AGENTS.md"
  "docs/feature-discovery-plan.md"
  "docs/capability-mapping-plan.md"
  "docs/input-json-concept-mapping-plan.md"
  "docs/offline-analyzer/feature-inventory.md"
  "docs/offline-analyzer/capability-concepts.md"
  "docs/offline-analyzer/feature-requirements-draft.md"
)

for file in "${required_files[@]}"; do
  test -f "$file"
done

# The concept and requirement drafts intentionally avoid final raw path mappings.
! rg -n 'Device\.|InternetGatewayDevice\.' \
  docs/offline-analyzer/capability-concepts.md \
  docs/offline-analyzer/feature-requirements-draft.md

rg -n 'Milestone 1|Acceptance criteria|Next Command' docs/input-json-concept-mapping-plan.md >/dev/null

git diff --check
