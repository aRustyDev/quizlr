# Issue: E2E tests fail with ERR_CONNECTION_REFUSED [RESOLVED]

## Problem
`just test-e2e` fails with connection refused error when trying to access http://localhost:3000

## Steps to Reproduce
1. Run `just test-e2e`
2. Tests fail with connection errors

## Expected Behavior
E2E tests should pass or at least connect to the application.

## Actual Behavior
```
Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:3000/
```

## Investigation
- All tests were failing because they couldn't connect to localhost:3000
- The Playwright config had webServer setup to auto-start trunk serve
- Port 3000 was already occupied by another application (Docker)
- Trunk's default port is 8080, but the config was trying to use 3000

## Solution Applied
1. Changed the application port from 3000 to 3001 to avoid conflicts
2. Updated `e2e/playwright.config.ts`:
   - Changed baseURL to use port 3001
   - Updated webServer command to include `--port 3001`
   - Updated webServer url to use port 3001
3. Updated `justfile` dev command to use port 3001 for consistency

## Result
All 55 E2E tests now pass successfully across all browsers (Chrome, Firefox, Safari, Mobile Chrome, Mobile Safari)

## Status
RESOLVED - E2E tests now run successfully with automatic server startup