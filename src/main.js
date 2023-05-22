// const {invoke} = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let hubStatus = true;

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
        name: greetInputEl.value,
    });
}

window.addEventListener("DOMContentLoaded", () => {
    greetInputEl = document.querySelector("#greet-input");
    greetMsgEl = document.querySelector("#greet-msg");

    // modal processing

    const settingsButton = document.querySelector("#settings-button");
    settingsButton.addEventListener("click", openModal);

    const closeButton = document.querySelector("#close-settings-modal");
    closeButton.addEventListener("click", closeModal);

    const openPackButton = document.querySelector("#open-pack");
    openPackButton.addEventListener("click", openModalPackInfo);

    const closePackInfoButton = document.querySelector(
        "#close-pack-info-modal"
    );
    closePackInfoButton.addEventListener("click", closeModallPackInfo);

    const first = document.querySelector("#term-one");
    first.addEventListener("click", selectImage);

    document
        .querySelector("#discover-terminals")
        .addEventListener("click", discover);

    document
        .querySelector("#serial-port-menu")
        .addEventListener("change", serialPortSelectHandler);
});

console.log("test");

async function openModal() {
    const modalContainer = document.querySelector("#settings-modal");
    // Serial data obtaining
    // const result = await invoke("discover_serial_ports");

    // console.info("openModal: result = " + result);

    // Fill serial port menu
    const serialPortMenu = document.querySelector("#serial-port-menu");
    serialPortMenu.innerHTML = "";

    let optionElement = document.createElement("option");
    optionElement.innerText = "Select port";
    serialPortMenu.appendChild(optionElement);

    //   result.forEach((portName) => {
    //     var optionElement = document.createElement("option");
    //     optionElement.innerText = portName;

    //     serialPortMenu.appendChild(optionElement);
    //   });

    setHubStatus(hubStatus);
    hubStatus = !hubStatus;

    modalContainer.style.display = "block";
    modalContainer.offsetHeight;
    modalContainer.style.opacity = 1;
}

function closeModal() {
    const modalContainer = document.querySelector("#settings-modal");

    modalContainer.style.opacity = 0;

    setTimeout(function () {
        modalContainer.style.display = "none";
    }, 500);
}

async function openModalPackInfo() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");
    const serialPortMenu = document.querySelector("#serial-port-menu");
    serialPortMenu.innerHTML = "";

    let optionElement = document.createElement("option");
    optionElement.innerText = "Select port";
    serialPortMenu.appendChild(optionElement);

    setHubStatus(hubStatus);
    hubStatus = !hubStatus;

    modalPackInfoContainer.style.display = "block";
    // modalPackInfoContainer.offsetHeight;
    modalPackInfoContainer.style.opacity = 1;
}

function closeModallPackInfo() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");

    modalPackInfoContainer.style.opacity = 0;

    setTimeout(function () {
        modalPackInfoContainer.style.display = "none";
    }, 500);
}

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

function setHubStatus(status) {
    const hubStatus = document.querySelector(".hub-status");

    if (status === "Detected") {
        hubStatus.textContent = "Detected";
        hubStatus.className = "hub-status detected";
    } else {
        hubStatus.textContent = "Not detected";
        hubStatus.className = "hub-status not-detected";
    }
}

async function discover() {
    const channelIdObject = document.querySelector("#radio-channel");
    const result = await invoke("discover_terminals", {
        channelId: parseInt(channelIdObject.value),
    });

    console.info("result = " + result);
}

async function serialPortSelectHandler(event) {
    // Get the selected option value
    const selectedOption = event.target.value;

    // Perform actions based on the selected option
    console.log("Selected option:", selectedOption);

    const result = await invoke("open_selected_port", { path: selectedOption });

    console.info("serialPortSelectHandler: result = " + result);

    setHubStatus(result);
}

setHubStatus();
