# GitHub Actions Workflows

This directory contains the automated CI/CD workflow for the glin-sdk-rust workspace.

## Workflow: CI and Publish

**File**: `ci-publish.yml`

A unified workflow that handles both continuous integration checks and automated publishing to crates.io.

### What It Does

The workflow runs in three sequential stages:

#### Stage 1: Quality Checks (Always Runs)

Runs on every push and pull request to `main` or `develop` branches:

1. **Format Check** - Ensures code follows Rust formatting standards (`cargo fmt`)
2. **Clippy Lints** - Catches common mistakes and enforces best practices (`cargo clippy`)
3. **Test Suite** - Runs all tests on stable and beta Rust versions
4. **Documentation** - Validates that documentation builds without warnings
5. **Security Audit** - Checks for known security vulnerabilities

All jobs run in parallel for speed. **Publishing only happens if ALL checks pass.**

#### Stage 2: Detect Changes (Main Branch Only)

Only runs on pushes to `main` branch (or manual workflow dispatch):

- Automatically detects which packages changed in the last commit
- Determines if any packages need to be published
- Skips publish if no package changes detected (e.g., README-only changes)

#### Stage 3: Version and Publish (After All Checks Pass)

Only runs if:
- All quality checks passed
- Push is to `main` branch (or manual trigger)
- At least one package changed

For each changed package:
1. **Auto-version bump** based on commit message:
   - `fix:` or `patch:` → patch version (0.1.2 → 0.1.3)
   - `feat:` or `minor:` → minor version (0.1.2 → 0.2.0)
   - `breaking:` or `major:` → major version (0.1.2 → 1.0.0)
2. **Update dependencies** in other workspace packages
3. **Publish to crates.io**
4. **Commit version bump** back to repository
5. **Create Git tag** (e.g., `glin-contracts-v0.1.3`)
6. **Create GitHub Release** with changelog

### When It Runs

| Event | Quality Checks | Publish |
|-------|----------------|---------|
| Push to `main` | ✅ Yes | ✅ Yes (if packages changed) |
| Push to `develop` | ✅ Yes | ❌ No |
| Pull Request | ✅ Yes | ❌ No |
| Manual Trigger | ✅ Yes | ✅ Yes |

### Setup Requirements

Add this secret to your GitHub repository:

- `CARGO_REGISTRY_TOKEN` - Token from crates.io for publishing
  - Generate at: https://crates.io/me/tokens
  - Add at: Repository Settings → Secrets and variables → Actions → New repository secret

### Usage Examples

#### Automatic Publishing (Recommended)

Just commit and push to `main` with semantic commit messages:

```bash
# This will publish a patch version (0.1.2 → 0.1.3)
git commit -m "fix(glin-contracts): resolve AccountId encoding bug"
git push origin main

# This will publish a minor version (0.1.2 → 0.2.0)
git commit -m "feat(glin-client): add batch RPC support"
git push origin main

# This will publish a major version (0.1.2 → 1.0.0)
git commit -m "breaking(glin-types): change ExtrinsicInfo API"
git push origin main
```

The workflow will:
1. Run all quality checks
2. Detect which packages changed
3. Auto-bump versions based on commit prefix
4. Publish to crates.io
5. Create tags and releases

#### Manual Publishing

Use GitHub's "Actions" tab:

1. Go to: Repository → Actions → "CI and Publish"
2. Click: "Run workflow"
3. Select:
   - Package: Choose specific package or "all"
   - Version bump: Choose patch/minor/major
4. Click: "Run workflow"

Quality checks still run first - publish only happens if they pass.

### Benefits of Unified Workflow

✅ **No duplicate work** - Tests run once, not twice
✅ **Enforced order** - Publish only after all checks pass
✅ **No race conditions** - Sequential stages prevent conflicts
✅ **Works everywhere** - PRs get tested, only main publishes
✅ **Efficient caching** - Shared cargo cache across jobs
✅ **Safe** - Impossible to publish broken code

### Troubleshooting

#### Publishing didn't trigger after push

Check:
- Was the push to `main` branch?
- Did any package files actually change?
- Did all quality checks pass?

View workflow logs: Repository → Actions → Select the workflow run

#### Publish failed with "crate already exists"

The version wasn't bumped properly. Check:
- Commit message uses correct prefix (`fix:`, `feat:`, `breaking:`)
- Version in Cargo.toml is different from crates.io

#### Quality checks failed

Fix the issues locally first:

```bash
# Check formatting
cargo fmt --all -- --check

# Fix formatting automatically
cargo fmt --all

# Run clippy
cargo clippy --workspace --all-features -- -D warnings

# Run tests
cargo test --workspace --all-features
```

Then commit and push the fixes.

### Package Dependency Order

The workflow publishes packages in the matrix, which may run in parallel. For sequential dependencies:

1. `glin-types` (no dependencies)
2. `glin-client` (depends on glin-types)
3. `glin-contracts` (depends on glin-client, glin-types)
4. `glin-indexer` (depends on glin-client, glin-types)

If multiple packages change, they're published independently. Ensure inter-package version updates are committed together.

### Best Practices

1. **Use semantic commit messages** for automatic versioning
2. **Test locally** before pushing (`cargo test && cargo clippy`)
3. **Group related changes** in one commit when updating multiple packages
4. **Let CI run** - don't force-push while workflows are running
5. **Review workflow logs** if publish fails

### Workflow Performance

Typical run times:
- Quality checks (parallel): ~3-5 minutes
- Publishing (per package): ~2-3 minutes
- Total (1 package changed): ~5-8 minutes
- Total (all packages): ~5-15 minutes (parallel matrix)

Caching significantly speeds up subsequent runs.
