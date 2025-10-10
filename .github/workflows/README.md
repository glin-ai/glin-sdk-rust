# CI/CD Workflows for GLIN SDK Rust

This directory contains GitHub Actions workflows for automated testing, versioning, and publishing.

## Workflows

### 1. CI (`ci.yml`) ✅
**Triggers:** Push to `main`/`develop`, Pull Requests

**Purpose:** Continuous Integration - validates code quality before merging

**Jobs:**
- **Test**: Runs full test suite on stable and beta Rust
- **Fmt**: Checks code formatting with `rustfmt`
- **Clippy**: Lints code for common mistakes
- **Docs**: Validates documentation builds
- **Security Audit**: Scans for known vulnerabilities

### 2. Publish (`publish.yml`) 🚀
**Triggers:**
- **Automatic**: Push to `main` when packages change
- **Manual**: Workflow dispatch with package and version selection

**Purpose:** Automated versioning and publishing to crates.io

**Features:**
- ✅ Auto-detects changed packages
- ✅ Bump versions (patch/minor/major)
- ✅ Updates inter-package dependencies
- ✅ Runs full test suite before publishing
- ✅ Publishes to crates.io
- ✅ Creates Git tags and GitHub releases
- ✅ Commits version bumps back to repo

## Setup Requirements

### 1. GitHub Secrets
Add these secrets in repository settings (`Settings` → `Secrets and variables` → `Actions`):

```
CARGO_REGISTRY_TOKEN    # Required for crates.io publishing
CODECOV_TOKEN          # Optional: for code coverage reports
```

#### Getting CARGO_REGISTRY_TOKEN:
1. Go to https://crates.io/settings/tokens
2. Click "New Token"
3. Name it "GitHub Actions"
4. Select scopes: `publish-new`, `publish-update`
5. Copy the token and add to GitHub secrets

### 2. Repository Permissions
Ensure the workflow has write permissions:
- `Settings` → `Actions` → `General` → `Workflow permissions`
- Select "Read and write permissions"

## Usage

### Automatic Publishing (Recommended)

When you push changes to `main`:

1. **Make your changes** in any package (e.g., fix AccountId bug in `glin-contracts`)

2. **Commit with semantic message**:
   ```bash
   git commit -m "fix: AccountId encoding for ink! contracts"
   # or
   git commit -m "feat: add new contract deployment method"
   # or
   git commit -m "breaking: change API signature"
   ```

3. **Push to main**:
   ```bash
   git push origin main
   ```

4. **CI/CD automatically**:
   - Detects `glin-contracts` changed
   - Runs all tests
   - Bumps version based on commit message:
     - `fix:` → patch (0.1.2 → 0.1.3)
     - `feat:` → minor (0.1.2 → 0.2.0)
     - `breaking:` → major (0.1.2 → 1.0.0)
   - Updates dependencies in other packages
   - Publishes to crates.io
   - Creates git tag `glin-contracts-v0.1.3`
   - Creates GitHub release

### Manual Publishing

Use this when you need to publish a specific package or version:

1. Go to `Actions` → `Version and Publish to crates.io`

2. Click `Run workflow`

3. Select:
   - **Package**: Which package to publish (or "all")
   - **Version bump**: patch/minor/major

4. Click `Run workflow`

## Dependency Order

Packages are published in dependency order:

```
glin-types (no dependencies)
    ↓
glin-client (depends on glin-types)
    ↓
glin-contracts (depends on glin-types + glin-client)
    ↓
glin-indexer (depends on glin-types + glin-client)
```

## Version Management

### Current Strategy
All packages share the same workspace version in root `Cargo.toml`:

```toml
[workspace.package]
version = "0.1.3"  # All packages use this
```

### Inter-Package Dependencies
Use compatible version requirements:

```toml
glin-types = { version = "0.1", path = "../glin-types" }
```

This allows:
- `0.1.2` ✅
- `0.1.3` ✅
- `0.1.999` ✅
- `0.2.0` ❌ (would need manual update)

## Troubleshooting

### Publishing Fails: "version already exists"
- The version wasn't bumped properly
- Manually bump in `Cargo.toml` or use `cargo set-version`

### Tests Fail Before Publishing
- Fix the tests - workflow won't publish failing code
- Check CI logs for details

### Dependency Version Mismatch
- Update the version requirements in dependent packages
- Example: If `glin-types` is at `0.2.0`, update:
  ```toml
  glin-types = { version = "0.2", path = "../glin-types" }
  ```

### Manual Override
If you need to publish manually:

```bash
cd glin-contracts
cargo set-version 0.1.3
cargo publish
```

## Best Practices

### 1. Semantic Commit Messages
Use conventional commits for auto-versioning:

```bash
# Patch release (0.1.2 → 0.1.3)
git commit -m "fix: resolve AccountId encoding bug"

# Minor release (0.1.2 → 0.2.0)
git commit -m "feat: add batch deployment support"

# Major release (0.1.2 → 1.0.0)
git commit -m "breaking: change deploy() API signature"
```

### 2. Test Before Pushing
Always run locally before pushing:

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

### 3. Update CHANGELOG
Maintain a CHANGELOG.md in each package documenting changes.

### 4. Review Dependencies
When bumping a base package (like `glin-types`), verify all dependent packages still work.

## Monitoring

### Check Workflow Status
- Go to `Actions` tab in GitHub
- View running/completed workflows
- Check logs for failures

### Verify Published Version
```bash
cargo search glin-contracts --limit 1
# Should show: glin-contracts = "0.1.3"
```

### View Release
- Go to `Releases` in GitHub
- Each published version has a release with:
  - Tag (e.g., `glin-contracts-v0.1.3`)
  - Installation instructions
  - Commit hash

## Future Improvements

- [ ] Add changelog generation from commits
- [ ] Add performance benchmarking
- [ ] Add integration tests with live network
- [ ] Add Docker image publishing
- [ ] Add npm package publishing for wasm builds
- [ ] Add notification to Slack/Discord on publish
