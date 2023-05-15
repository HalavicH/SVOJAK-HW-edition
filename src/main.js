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

    const settingsButton = document.querySelector("#settings-button");
    settingsButton.addEventListener("click", openModal);

    const closeButton = document.querySelector("#close-settings-modal");
    closeButton.addEventListener("click", closeModal);
});

console.log("test");


