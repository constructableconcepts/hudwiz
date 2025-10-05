# hudwiz Integration Guide

This guide provides a comprehensive overview for developers on how to integrate a backend service with the `hudwiz` frontend client. It covers the core architectural concepts, the API contract your backend must adhere to, and how to extend the frontend with your own custom components.

## Table of Contents
1.  [Architectural Overview](#1-architectural-overview)
    -   [Client-Server Relationship](#11-client-server-relationship)
    -   [The `ApiService` Trait](#12-the-apiservice-trait)
    -   [The `RealtimeProvider` Trait](#13-the-realtimeprovider-trait)
    -   [The `mock_api` Feature Flag](#14-the-mock_api-feature-flag)
2.  [Backend API Specification](#2-backend-api-specification)
    -   [Authentication API](#21-authentication-api)
    -   [Real-time Communication API](#22-real-time-communication-api)
3.  [Frontend Extension](#3-frontend-extension)
    -   [Adding a Custom Component](#31-adding-a-custom-component)
4.  [Full Integration Workflow](#4-full-integration-workflow)

---

## 1. Architectural Overview

### 1.1. Client-Server Relationship
`hudwiz` is designed with a decoupled architecture. The frontend client is a standalone Single Page Application (SPA) built with Yew, while the backend is a separate service that provides data and authentication. This separation allows for a great deal of flexibility in how you build and deploy your application.

Your backend's primary responsibilities are:
-   Serving the static frontend assets.
-   Providing a set of RESTful API endpoints for authentication.
-   (Optionally) Providing a WebSocket or WebTransport endpoint for real-time communication.

### 1.2. The `ApiService` Trait
All HTTP communication from the frontend is handled through the `ApiService` trait, defined in `hudwiz/frontend/src/services/api_service.rs`. This trait abstracts the details of HTTP requests, providing a clean interface for components to interact with the backend.

### 1.3. The `RealtimeProvider` Trait
Real-time communication is handled by the `RealtimeProvider` trait, which can be implemented using WebSockets or WebTransport. This service is responsible for establishing a connection with the server and handling incoming messages for dynamic content updates.

### 1.4. The `mock_api` Feature Flag
For ease of development, the `hudwiz` frontend includes a `mock_api` feature flag. When enabled, the application uses a `MockApiService` and `MockRealtimeProvider` that return hardcoded data without making any real network requests.

**This is the default behavior when running the system tests.**

To connect to a real backend, you must build the frontend **without** this feature flag.

**Build command for development (with mock data):**
```bash
wasm-pack build --target web --out-name wasm --out-dir ./dist --release -- --features mock_api
```

**Build command for production (with a real backend):**
```bash
wasm-pack build --target web --out-name wasm --out-dir ./dist --release
```

---

## 2. Backend API Specification

To integrate with `hudwiz`, your backend must implement the following API endpoints.

### 2.1. Authentication API

`hudwiz` uses a standard token-based authentication flow.

#### **`POST /api/auth/login`**
*   **Description:** Authenticates a user and returns a session token.
*   **Request Body:**
    ```json
    {
      "username": "user@example.com",
      "password": "user_password"
    }
    ```
*   **Success Response (200 OK):**
    ```json
    {
      "token": "your_jwt_here"
    }
    ```
*   **Failure Response (401 Unauthorized):**
    ```json
    {
      "error": "Invalid credentials"
    }
    ```

#### **`GET /api/auth/user`**
*   **Description:** Retrieves information about the currently authenticated user.
*   **Authentication:** Requires a valid `Bearer` token in the `Authorization` header.
*   **Success Response (200 OK):**
    ```json
    {
      "id": "user-id-123",
      "name": "Jules Verne",
      "email": "jules@example.com"
    }
    ```
*   **Failure Response (401 Unauthorized):**
    ```json
    {
      "error": "Invalid or expired token"
    }
    ```

#### **`POST /api/auth/logout`**
*   **Description:** Invalidates the user's session token on the backend.
*   **Response:** A successful (2xx) status code.

### 2.2. Real-time Communication API

The `hudwiz` client will attempt to connect to a WebSocket or WebTransport endpoint at `/ws` to receive real-time updates.

#### **Endpoint: `/ws`**
*   **Protocol:** WebSocket or WebTransport.
*   **Message Format:** The server should send JSON messages to the client to trigger dynamic content updates. The message must have a `type` field that corresponds to an `AppAction` variant in the frontend's state service.

**Example Messages:**

*   **Set Main Content:**
    ```json
    {
      "type": "SetMainContent",
      "payload": { "IFrame": "https://www.openstreetmap.org/export/embed.html" }
    }
    ```

*   **Set Background Content:**
    ```json
    {
      "type": "SetBackgroundContent",
      "payload": { "SolidColor": "#1a1a1a" }
    }
    ```

---

## 3. Frontend Extension

### 3.1. Adding a Custom Component

One of the most powerful features of `hudwiz` is the ability to add your own custom components to the UI.

**Step 1: Create Your Yew Component**
Create a new `.rs` file in `hudwiz/frontend/src/components/` and define your Yew component.

**Step 2: Register the Component Type**
Open `hudwiz/frontend/src/services/config_service.rs` and add a new variant for your component to the `ComponentType` enum.

```rust
// In hudwiz/frontend/src/services/config_service.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    // ... other components
    MyCustomWidget, // <-- Add your new component
}
```

**Step 3: Map the Component in the Renderer**
Open `hudwiz/frontend/src/components/main_layout.rs` and add your component to the `render_component` function.

```rust
// In hudwiz/frontend/src/components/main_layout.rs
use crate::components::my_custom_widget::MyCustomWidget;

fn render_component(component_type: &ComponentType) -> Html {
    match component_type {
        // ... other mappings
        ComponentType::MyCustomWidget => html! { <MyCustomWidget /> },
    }
}
```

**Step 4: Configure the Layout**
Finally, update `hudwiz/frontend/static/config.json` to tell `hudwiz` where to render your component.

```json
{
  "layout": {
    "main_panel": { "component": "MyCustomWidget" }
  }
}
```

---

## 4. Full Integration Workflow

1.  **Implement the Backend API:** Create the required authentication and real-time endpoints on your backend server.
2.  **Build the Frontend:** Build the `hudwiz` client **without** the `--features mock_api` flag.
3.  **Serve the Frontend:** Configure your backend server to serve the static files from the `hudwiz/frontend/dist` directory. Ensure that all routes that are not static files serve the `index.html` file.
4.  **Configure `hudwiz`:** Modify `config.json` to arrange the UI and include any custom components you have created.
5.  **Launch:** Start your backend server and navigate to the root URL to see your integrated `hudwiz` application in action.