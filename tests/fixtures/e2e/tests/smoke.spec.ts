import { test, expect } from "@playwright/test";

test("smoke", async () => {
  expect(1 + 1).toBe(2);
});

test("database url well formed when set", async () => {
  const url = process.env.DATABASE_URL;
  if (url) {
    expect(url.startsWith("postgres://")).toBe(true);
  }
});
