## About the project

* I'm building a SvelteKit based stand-alone desktop application using Tauri.
* The JS part of this Tauri app requires very specific integrations with an external browser based app.
* These integrations need to be implemented in Rust and available as invoked handlers. See details: [Calling Rust from the frontend](https://tauri.app/v1/guides/features/command).
* The Rust implementation should be able to launch a Chrome browser window that is controlled using DevTools (using [chromiumoxide](https://crates.io/crates/chromiumoxide), or better alternative).
* It should be able to identify Chrome windows/popups opened/launched under its control.
* On any specific window/popup under its control, it should be able to:
  * Await content changes.
  * Find DOM elements.
  * Fill in input fields.
  * Click on buttons.
  * Retrieve params from the URL.

## Developing

* Please see https://tauri.app/v1/guides/getting-started/prerequisites and ensure you have a compatible Rust development environment for this app.
* For JS part: `node` version `18.20.3` (v18) with `npm` version `10.7.0` (if using `nvm` then just install and use `18`).
* Install dependencies for both JS and Rust parts.
* To launch the Tauri app in development mode

```bash
npm run tauri dev
```

## Acceptance criteria

* When the app is running, user sees two buttons: "Login" and "Add Margin" which is rendered from the JS part.
* Clicking on "Login" should perform the steps for the "Login" operation
* Clicking on "Add Margin" should perform the steps for the "Add Margin" operation
* Safe to assume: Add Margin will be done ONLY after Login.
