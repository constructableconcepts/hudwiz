# Release Notes: hudwiz v0.0.3

## Introduction

We are thrilled to announce the official v0.0.3 release of hudwiz! This release marks a significant milestone for the project, introducing a host of major features, a complete visual overhaul, and substantial under-the-hood refactoring to improve stability and scalability.

Our goal with this release was to enhance the user experience, provide greater customization options, and solidify the application's architecture for future development.

## Major New Features

###🎨 New Theming System
- **Six New Themes:** We've introduced six beautiful new themes to customize your development environment. You can now choose from a variety of light and dark themes to suit your preferences.
  - **Dark Themes:** Dracula, Nord, Monokai, Cyberpunk
  - **Light Themes:** GitHub Light, Solarized Light
- **Theme Switcher:** A new theme switcher has been added to the left sidebar, allowing you to instantly switch between themes.

### ✨ Redesigned Landing Page
- The application's landing page has been completely redesigned with a modern, professional, GitHub-inspired aesthetic.
- The new design features a prominent hero image showcasing the application, a clear call-to-action, and improved navigation.

## Core Architectural Improvements & Refactoring

### State Management Overhaul
- The application's state management has been refactored to use a centralized `UseReducerHandle`. This resolves a critical bug that prevented UI updates and aligns the codebase with Yew best practices, ensuring a more stable and predictable experience.

### Screen Region Renaming
- All internal screen region identifiers have been renamed to be more descriptive and intuitive (e.g., `left_region` is now `sidebar_left`). This change has been propagated throughout the entire frontend, including components, state, CSS, and configuration, making the codebase easier to understand and maintain.

### ContentType Unification
- The `BackgroundContentType` and `ContentType` enums have been unified into a single, streamlined `ContentType`. This simplification removes redundant code and makes the application's state easier to manage.

## Bug Fixes
- Fixed a critical state propagation issue that prevented theme changes from being applied to the UI.
- Resolved a CSS layout bug that caused HUD elements to overlap and block user interaction.
- Corrected multiple issues in the end-to-end test script to ensure reliable verification of the application's functionality.

We are incredibly excited about this release and believe it lays a strong foundation for the future of hudwiz. Thank you for your support!