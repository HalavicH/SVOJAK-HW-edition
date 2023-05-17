const {invoke} = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let hubStatus = true;

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

    const first = document.querySelector("#term-one");
    first.addEventListener("click", selectImage);

    document.querySelector("#discover-terminals")
            .addEventListener("click", discover);

    document.querySelector("#serial-port-menu")
            .addEventListener("change", serialPortSelectHandler);

});

console.log("test");

async function openModal() {
    const modalContainer = document.querySelector("#settings-modal");

    // Serial data obtaining
    var result = await invoke("discover_serial_ports");

    console.info("openModal: result = " + result);

    // Fill serial port menu
    var serialPortMenu = document.querySelector("#serial-port-menu");

    result.forEach((portName) => {
        var optionElement = document.createElement("option");

        optionElement.innerText = portName;

        serialPortMenu.appendChild(optionElement);
    });


    setHubStatus(hubStatus);
    hubStatus = !hubStatus;


    modalContainer.style.display = "block";
    modalContainer.offsetHeight;
    modalContainer.style.opacity = 1;
};

function closeModal() {
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
            document.getElementById("term-one").src = e.target.result;
        };
        reader.readAsDataURL(file);
    });

    // Trigger a click event on the file input element
    fileInput.click();
}

function setHubStatus(isDetected) {
    var hubStatus = document.querySelector(".hub-status");

    if (isDetected) {
        hubStatus.textContent = "Detected";
        hubStatus.className = "hub-status detected";
    } else {
        hubStatus.textContent = "Not detected";
        hubStatus.className = "hub-status not-detected";
    }
}

async function discover() {
    var channelIdObject = document.querySelector("#radio-channel");
    var result = await invoke("discover_terminals",
                              {channelId: parseInt(channelIdObject.value)});

    console.info("result = " + result);
}

async function serialPortSelectHandler(event) {
    // Get the selected option value
    var selectedOption = event.target.value;

    // Perform actions based on the selected option
    console.log("Selected option:", selectedOption);

    var result = await invoke("open_selected_port", {path: selectedOption});

    console.info("serialPortSelectHandler: result = " + result);
}