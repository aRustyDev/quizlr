# TODOs

- [x] Verify robust `playwright` configuration - COMPLETED
  - Enhanced with better error reporting, screenshots, video recording, and CI-specific configurations
  - Added timeouts and retry configurations
  - Fixed port conflicts (now using port 3001)
  
- [x] justfile target docs-deploy - COMPLETED
  - Added target for GitHub Actions deployment
  - Prevents accidental local deployment
  
- [x] justfile target ci - COMPLETED
  - Added `ci` target that runs: lint, test, check, build, test-e2e, build-docs
  - Added `ci-quick` target for faster feedback without E2E tests
  - Fixed wasm-opt build issues by using debug builds for web frontend


