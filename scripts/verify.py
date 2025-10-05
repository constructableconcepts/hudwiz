import sys
import os
import logging
from playwright.sync_api import sync_playwright, Page, expect

def run_verification(page: Page, output_dir: str):
    """
    Navigates to the application, logs in, and captures a screenshot of the final UI state.
    """
    logging.info("Navigating to the application...")
    page.goto("http://localhost:8080")

    # Use a consistent username for this test run
    username = "testuser@example.com"
    password = "password123"

    # Go to signup page
    page.goto("http://localhost:8080/signup")
    expect(page.locator(".auth-container")).to_be_visible(timeout=15000)

    # Sign up
    logging.info("Testing signup...")
    page.locator("#email").fill(username)
    page.locator("#password").fill(password)
    page.locator("#confirm-password").fill(password)
    page.locator("button[type='submit']").click()
    page.wait_for_url("http://localhost:8080/login", timeout=15000)
    logging.info("Successfully signed up and redirected to login page.")

    # Log in
    logging.info("Testing username/password login...")
    page.locator("#email").fill(username)
    page.locator("#password").fill(password)
    page.locator("button[type='submit']").click()
    expect(page.locator("#main-layout")).to_be_visible(timeout=15000)
    logging.info("Successfully logged in and redirected to app page.")

    logging.info("Waiting for the HUD layout to be visible...")
    main_layout = page.locator("#main-layout")
    expect(main_layout).to_be_visible(timeout=15000)
    page.wait_for_timeout(1000)

    # Verify image is visible
    logging.info("Verifying image content...")
    image_content = page.locator(".content-image")
    expect(image_content).to_be_visible(timeout=5000)

    logging.info("Capturing screenshot of final UI...")
    screenshot_path = f"{output_dir}/main_page_final.png"
    page.screenshot(path=screenshot_path)
    logging.info(f"Screenshot saved to {screenshot_path}")


def main():
    # Set up logging
    log_level = os.environ.get("LOG_LEVEL", "INFO").upper()
    logging.basicConfig(
        level=log_level,
        format="%(asctime)s - %(levelname)s - %(message)s",
    )

    if len(sys.argv) < 2:
        logging.error("Usage: python verify.py <output_directory>")
        sys.exit(1)

    output_dir = sys.argv[1]

    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Capture console logs and route them to our logger
        page.on("console", lambda msg: logging.debug(f"BROWSER LOG: {msg.text}"))

        try:
            run_verification(page, output_dir)
            logging.info("Verification script completed successfully.")
        except Exception as e:
            logging.error(f"An error occurred during verification: {e}", exc_info=True)
            error_path = f"{output_dir}/error.png"
            page.screenshot(path=error_path)
            logging.error(f"Error screenshot saved to {error_path}")
            raise e
        finally:
            browser.close()

if __name__ == "__main__":
    main()