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
        const modal = document.querySelector("#settingsModal");
        modal.style.display = "block";
    };

    const closeModal = () => {
        const modal = document.querySelector("#settingsModal");
        modal.style.display = "none";
    };

    const settingsButton = document.querySelector("#settings-button");
    settingsButton.addEventListener("click", openModal);

    const closeButton = document.querySelector("#close-settings-modal");
    closeButton.addEventListener("click", closeModal);
});

console.log("test");


