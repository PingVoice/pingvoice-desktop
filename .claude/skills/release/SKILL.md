---
name: release
description: Create a new PingVoice release with auto-versioning. Usage: /release [patch|minor|major]. Defaults to patch bump.
allowed-tools: Bash, Read, Edit
---

# Release Skill

Create a release by auto-calculating the next version, updating files, committing, tagging, and pushing.

## Arguments

$ARGUMENTS should be the bump type:
- `patch` (default): 1.0.0 → 1.0.1
- `minor`: 1.0.0 → 1.1.0
- `major`: 1.0.0 → 2.0.0

If no argument provided, defaults to `patch`.

## Process

1. **Determine bump type**: Parse $ARGUMENTS. Valid values: `patch`, `minor`, `major`. Default to `patch` if empty or unrecognized.

2. **Get current version**:
   ```bash
   git tag -l 'v*' --sort=-v:refname | head -1
   ```
   If no tags exist, start from `0.0.0`.

3. **Calculate next version**: Apply bump to current version.
   - patch: increment Z (1.2.3 → 1.2.4)
   - minor: increment Y, reset Z (1.2.3 → 1.3.0)
   - major: increment X, reset Y and Z (1.2.3 → 2.0.0)

4. **Pre-flight checks**:
   - Verify on `main` branch: `git branch --show-current`
   - Verify clean working directory: `git status --porcelain` should be empty
   - Verify tag doesn't exist: `git tag -l vX.Y.Z` should return nothing
   - Fetch and check remote: `git fetch origin && git status`

5. **Update version files**:
   - `Cargo.toml`: Update `version = "X.Y.Z"` line
   - `tauri.conf.json`: Update `"version": "X.Y.Z"` field

6. **Commit and tag**:
   ```bash
   git add Cargo.toml tauri.conf.json
   git commit -m "Bump version to X.Y.Z"
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   ```

7. **Push**:
   ```bash
   git push origin main
   git push origin vX.Y.Z
   ```

8. **Report**: Show version bump summary and GitHub Actions URL.

## Example

User: `/release minor`

Current version: v1.2.3
Next version: v1.3.0

Output:
- Bumping minor version: 1.2.3 → 1.3.0
- Updated Cargo.toml version to 1.3.0
- Updated tauri.conf.json version to 1.3.0
- Created commit: "Bump version to 1.3.0"
- Created tag: v1.3.0
- Pushed to origin
- Monitor build: https://github.com/OWNER/REPO/actions
