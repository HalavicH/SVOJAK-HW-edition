import { setupSettingsModalCallbacks } from "./modal/settings-modal.js";
import { setupPackInfoCallbacks } from "./modal/pack-info-modal.js";

window.addEventListener("DOMContentLoaded", () => {
    setupSettingsModalCallbacks();
    setupPackInfoCallbacks();
});
