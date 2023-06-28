import {
    openSettingsModal,
    closeSettingsModal,
    serialPortSelectHandler,
    handleSetHubRadioChannel, handleDiscoverTerminals
} from "./modal/settings-modal.js";
import {openPackInfoModal, closePackInfoModal, handleStartTheGame, closePackErrorModal} from "./modal/pack-info-modal.js";
import {setupHubDebugCallbacks} from "./modal/hub-debug-modal.js";
const { convertFileSrc } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", () => {
    // modal processing

    // const settingsBtn = document.querySelector("#settings-button");
    // settingsBtn.addEventListener("click", openSettingsModal);

    document
        .querySelector("#settings-button")
        .addEventListener("click", openSettingsModal);

    document
        .querySelector("#close-settings-modal")
        .addEventListener("click", closeSettingsModal);

    setupHubDebugCallbacks();

    document
        .querySelector("#open-pack")
        .addEventListener("click", openPackInfoModal);

    document
        .querySelector("#close-pack-info-modal")
        .addEventListener("click", closePackInfoModal);

    // TODO: REWORK
    document
        .querySelector("#term-one")
        .addEventListener("click", selectImage);

    document
        .querySelector("#set-hub-radio-channel")
        .addEventListener("click", handleSetHubRadioChannel);

    document
        .querySelector("#refresh-terminals-btn")
        .addEventListener("click", handleDiscoverTerminals);

    document
        .querySelector("#serial-port-menu")
        .addEventListener("change", serialPortSelectHandler);

    document
        .querySelector("#start-the-game")
        .addEventListener("click", handleStartTheGame);

    document
        .querySelector("#pack-error-ok-btn")
        .addEventListener("click", closePackErrorModal)

    document
        .querySelector("#pack-error-close-modal")
        .addEventListener("click", closePackErrorModal)
});


function selectImage() {
    // Create an input element of type "file"
    const fileInput = document.createElement("input");
    fileInput.type = "file";

    // Set the accept attribute to filter only image files
    fileInput.accept = "image/*";

    // Add an event listener to handle file selection
    fileInput.addEventListener("change", function (event) {
        const file = event.target.files[0];

        // Read the selected image file
        const reader = new FileReader();
        reader.onload = function (e) {
            // Update the image cell with the selected image
            document.getElementById("term-one").src = e.target.result;
        };
        reader.readAsDataURL(file);
    });

    // Trigger a click event on the file input element
    fileInput.click();
}
