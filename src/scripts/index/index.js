import {
    serialPortSelectHandler,
    handleSetHubRadioChannel, handleDiscoverTerminals, setupSettingsModalCallbacks
} from "./modal/settings-modal.js";
import {
    openPackInfoModal,
    closePackInfoModal,
    handleStartTheGame,
    closePackErrorModal
} from "./modal/pack-info-modal.js";

window.addEventListener("DOMContentLoaded", () => {
    // modal processing
    setupSettingsModalCallbacks();

    document
        .querySelector("#open-pack")
        .addEventListener("click", openPackInfoModal);

    document
        .querySelector("#close-pack-info-modal")
        .addEventListener("click", closePackInfoModal);

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
