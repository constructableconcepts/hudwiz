# hudwiz User Guide

Welcome to the `hudwiz` User Guide. This document provides a comprehensive overview of how to configure, use, and extend the `hudwiz` client. Whether you're looking to integrate `hudwiz` into an existing project or build a new application from the ground up, this guide will provide you with the information you need to get started.

## Table of Contents
1.  [Introduction](#1-introduction)
2.  [Core Concepts](#2-core-concepts)
    -   [Layout System](#21-layout-system)
    -   [Component-Driven UI](#22-component-driven-ui)
    -   [Dynamic Content](#23-dynamic-content)
3.  [Configuration](#3-configuration)
    -   [The `config.json` File](#31-the-configjson-file)
    -   [Defining Regions](#32-defining-regions)
    -   [Available Components](#33-available-components)
4.  [Key Features](#4-key-features)
    -   [Theming Engine](#41-theming-engine)
    -   [Integrated Chat](#42-integrated-chat)
    -   [Bento Grid Layout](#43-bento-grid-layout)
    -   [Animation Service](#44-animation-service)
    -   [Component Blocks](#45-component-blocks)
5.  [Advanced Topics](#5-advanced-topics)
    -   [Creating Custom Components](#51-creating-custom-components)
    -   [Server Integration](#52-server-integration)

---

## 1. Introduction

`hudwiz` is a highly configurable, multi-region web application framework built with Rust and Yew. It is designed to accelerate the creation of complex, real-time Heads-Up Display (HUD) interfaces by providing a flexible layout system, a rich set of built-in components, and a powerful theming engine.

This guide will walk you through the core concepts of `hudwiz`, show you how to configure the UI to meet your needs, and provide detailed explanations of its key features.

## 2. Core Concepts

### 2.1. Layout System

The `hudwiz` client is built around a powerful and flexible layout system that divides the screen into multiple, distinct regions. This modular approach allows for a clean separation of concerns and makes it easy to compose complex UIs.

The primary regions are:
-   **`app_bar`**: A full-width region at the top of the screen.
-   **`activity_bar`**: A full-width region at the bottom of the screen.
-   **`sidebar_left`**: A collapsible vertical region on the left.
-   **`sidebar_right`**: A collapsible vertical region on the right.
-   **`main_panel`**: The main content area of the application.
-   **`floating_toolbar`**: A toolbar that floats within the `main_panel`.
-   **Corner HUDs**: Four overlay regions in the corners of the `main_panel`: `hud_top_left`, `hud_top_right`, `hud_bottom_left`, and `hud_bottom_right`.

### 2.2. Component-Driven UI

The entire `hudwiz` UI is composed of individual, reusable components built with the Yew framework. Each region in the layout can be populated with a specific component, allowing you to mix and match UI elements to create the perfect interface for your application.

### 2.3. Dynamic Content

`hudwiz` is designed for dynamic, real-time applications. The content of the main panel and the application background can be changed on the fly by dispatching actions that update the client's state. This allows you to create rich, interactive experiences that respond to user input and server-side events.

## 3. Configuration

### 3.1. The `config.json` File

The layout and composition of the `hudwiz` UI are controlled by a single configuration file: `hudwiz/frontend/static/config.json`. This file is fetched by the client on startup and is used to determine which components to render in which regions.

### 3.2. Defining Regions

The `config.json` file uses a simple JSON structure to map components to regions. The `layout` object contains a key for each region you want to configure.

**Example `config.json`:**
```json
{
  "layout": {
    "app_bar": { "component": "AppBar" },
    "sidebar_left": { "component": "ControlsDrawer" },
    "sidebar_right": { "component": "DetailsDrawer" },
    "activity_bar": { "component": "UserDrawer" },
    "main_panel": { "component": "ChatHistoryView" }
  }
}
```

### 3.3. Available Components

The following components are available to be used in the `config.json` file:

| Component Name | Description | Available Properties |
| :--- | :--- | :--- |
| `AppBar` | The main application bar at the top of the screen. | `None` |
| `ControlsDrawer` | A collapsible drawer for the left sidebar, ideal for navigation and controls. | `None` |
| `DetailsDrawer` | A collapsible drawer for the right sidebar, suitable for inspectors or details panels. | `None` |
| `UserDrawer` | The main user interaction area at the bottom of the screen, containing the chat input and icon tray. | `None` |
| `ChatHistoryView` | A component for displaying chat messages, intended for the `main_panel`. | `None` |
| `ChatInputBar` | A component with a text input and send button for chat. | `None` |
| `BentoGrid` | A flexible, multi-sized grid layout component. | `children` |
| `DrawingToolbar` | A floating toolbar for drawing and annotation tools. | `None` |
| `TopLeftHud`, `TopRightHud`, `BottomLeftHud`, `BottomRightHud` | Overlay regions in the corners of the main panel. | `None` |

## 4. Key Features

### 4.1. Theming Engine

`hudwiz` includes a powerful theming engine that allows you to customize the application's appearance. The theme is controlled by CSS variables, making it easy to create new themes or modify existing ones.

The UI uses a Material Design-inspired elevation system to create a sense of depth and hierarchy. You can customize the elevation of each region by modifying the `--elevation-` variables in `hudwiz/frontend/style/main.css`.

#### Creating a Custom Theme

To create a new theme, you can add a new `[data-theme="your-theme-name"]` block to `main.css` and define your own set of color and elevation variables.

**Example Custom Theme:**
```css
[data-theme="my-custom-theme"] {
    --background-color: #2c3e50;
    --text-color: #ecf0f1;
    --primary-accent-color: #3498db;
    --elevation-1-bg: #34495e;
    /* ... and so on */
}
```

### 4.2. Integrated Chat

The integrated chat feature provides a seamless communication experience.

![Chat Interface](resources/main_page_expanded.png)

It consists of two components:
-   **`ChatHistoryView`**: Displays the conversation history.
-   **`ChatInputBar`**: Provides a text input and send button.

To enable the chat feature, place the `ChatHistoryView` in the `main_panel` and the `UserDrawer` (which contains the `ChatInputBar`) in the `activity_bar` region in your `config.json`.

### 4.3. Bento Grid Layout

The `BentoGrid` component allows you to create flexible, multi-sized grid layouts. It is composed of `BentoGrid` and `BentoGridItem` components.

**Example Usage:**
```rust
html! {
    <BentoGrid>
        <BentoGridItem col_span={2} row_span={1}><div>{"Component A"}</div></BentoGridItem>
        <BentoGridItem><div>{"Component B"}</div></BentoGridItem>
        <BentoGridItem row_span={2}><div>{"Component C"}</div></BentoGridItem>
    </BentoGrid>
}
```

### 4.4. Animation Service

`hudwiz` provides a simple animation service for creating engaging micro-interactions. The service allows you to apply CSS animations to components programmatically.

**Example Usage:**
```rust
use crate::services::animation_service;

animation_service::apply_animation(element, "pulse-animation", 200);
```

### 4.5. Component Blocks

The component block system provides a way to share and reuse pre-composed UI components. You can "eject" a block to copy its source code into your project for customization.

**To eject a block:**
```bash
./scripts/eject_block.sh <BlockName>
```

**Available Blocks:**
-   `LoginForm`
-   `UserProfileCard`

## 5. Advanced Topics

### 5.1. Creating Custom Components

You can create your own custom components and integrate them into the `hudwiz` layout. To do so, you will need to:
1.  Create a new Yew component.
2.  Add a new variant to the `ComponentType` enum in `hudwiz/frontend/src/services/config_service.rs`.
3.  Update the `render_component` function in `hudwiz/frontend/src/components/main_layout.rs` to map your new enum variant to your component.

### 5.2. Server Integration

For a full guide on integrating `hudwiz` with a backend server, please see the **[Integration Guide](integration_guide.md)**.