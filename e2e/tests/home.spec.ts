import { test, expect } from '@playwright/test';

test.describe('Home Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('has title', async ({ page }) => {
    await expect(page).toHaveTitle(/Quizlr/);
  });

  test('displays welcome message', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Quizlr');
    await expect(page.locator('h2')).toContainText('Welcome to Quizlr!');
  });

  test('has create quiz button', async ({ page }) => {
    const createButton = page.locator('button:has-text("Create Quiz")');
    await expect(createButton).toBeVisible();
    await expect(createButton).toHaveClass(/primary/);
  });

  test('has import content button', async ({ page }) => {
    const importButton = page.locator('button:has-text("Import Content")');
    await expect(importButton).toBeVisible();
    await expect(importButton).toHaveClass(/secondary/);
  });

  test('buttons are clickable', async ({ page }) => {
    const createButton = page.locator('button:has-text("Create Quiz")');
    const importButton = page.locator('button:has-text("Import Content")');
    
    await expect(createButton).toBeEnabled();
    await expect(importButton).toBeEnabled();
  });

  test('page loads without errors', async ({ page }) => {
    // Check for console errors
    const consoleErrors: string[] = [];
    page.on('console', (msg) => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });

    await page.waitForLoadState('networkidle');
    expect(consoleErrors).toHaveLength(0);
  });

  test('responsive design works', async ({ page }) => {
    // Desktop view
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('.app-container')).toBeVisible();

    // Tablet view
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('.app-container')).toBeVisible();

    // Mobile view
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('.app-container')).toBeVisible();
  });
});