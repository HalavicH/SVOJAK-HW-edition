const {invoke} = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {name: greetInputEl.value});
}

window.addEventListener("DOMContentLoaded", () => {
    greetInputEl = document.querySelector("#greet-input");
    greetMsgEl = document.querySelector("#greet-msg");

    // modal processing

    const settingsButton = document.querySelector("#settings-button");
    settingsButton.addEventListener("click", openModal);

    const closeButton = document.querySelector("#close-settings-modal");
    closeButton.addEventListener("click", closeModal);

    const first = document.querySelector(".term-name");
    first.addEventListener("click", selectImage);
});

console.log("test");

const openModal = () => {
    const modalContainer = document.querySelector("#settings-modal");

    modalContainer.style.display = "block";
    modalContainer.offsetHeight;
    modalContainer.style.opacity = 1;
};

const closeModal = () => {
    const modalContainer = document.querySelector("#settings-modal");

    modalContainer.style.opacity = 0;

    setTimeout(function() {
        modalContainer.style.display = "none";
    }, 500);
};
function selectImage() {
    // Create an input element of type "file"
    var fileInput = document.createElement("input");
    fileInput.type = "file";

    // Set the accept attribute to filter only image files
    fileInput.accept = "image/*";

    // Add an event listener to handle file selection
    fileInput.addEventListener("change", function(event) {
        var file = event.target.files[0];

        // Read the selected image file
        var reader = new FileReader();
        reader.onload = function(e) {
            // Update the image cell with the selected image
            document.getElementById("image-cell").src = e.target.result;
        };
        reader.readAsDataURL(file);
    });

    // Trigger a click event on the file input element
    fileInput.click();
}
