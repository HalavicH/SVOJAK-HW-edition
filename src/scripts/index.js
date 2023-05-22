const {invoke} = window.__TAURI__.tauri;

let hubStatus = true;

window.addEventListener("DOMContentLoaded", () => {
    // modal processing

    document
        .querySelector("#settings-button")
        .addEventListener("click", openSettingsModal);

    document
        .querySelector("#close-settings-modal")
        .addEventListener("click", closeSettingsModal);

    document
        .querySelector("#open-pack")
        .addEventListener("click", openPackInfoModal);

    document
        .querySelector("#close-pack-info-modal")
        .addEventListener("click", closePackInfoModal);

    document
        .querySelector("#term-one")
        .addEventListener("click", selectImage);

    document
        .querySelector("#discover-terminals")
        .addEventListener("click", discover);

    document
        .querySelector("#serial-port-menu")
        .addEventListener("change", serialPortSelectHandler);
});

/////////////    Open      ///////////////

function openModal(modal) {
    modal.style.display = "block";
    modal.offsetHeight;
    modal.style.opacity = 1;
}

//////////      CLose       /////////////

function closeModal(modal) {
    modal.style.opacity = 0;

    setTimeout(function () {
        modal.style.display = "none";
    }, 500);
}

async function openSettingsModal() {
    const modalContainer = document.querySelector("#settings-modal");
    openModal(modalContainer);
    const config = getSettingsConfig();

    setHubStatus(config.hubStatus);
    fillSerialPortMenu(config.availablePorts, config.hubPort);
    setRadioChannel(config.radioChannel);
    fillPlayersData(config.players);

}

function closeSettingsModal() {
    const modalContainer = document.querySelector("#settings-modal");

    closeModal(modalContainer);
}

async function openPackInfoModal() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");
    const serialPortMenu = document.querySelector("#serial-port-menu");
    serialPortMenu.innerHTML = "";

    let optionElement = document.createElement("option");
    optionElement.innerText = "Select port";
    serialPortMenu.appendChild(optionElement);

    setHubStatus(hubStatus);
    hubStatus = !hubStatus;

    openModal(modalPackInfoContainer);
}

function closePackInfoModal() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");

    closeModal(modalPackInfoContainer);
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
    const hubStatusElement = document.querySelector("#hub-status-field");

    if (status === "Detected") {
        hubStatusElement.className = "hub-status detected";
        hubStatusElement.innerText = "Detected";
    } else if (status === "UnknownDevice") {
        hubStatusElement.className = "hub-status unknown-device";
        hubStatusElement.innerText = "Unknown Device";
    } else {
        hubStatusElement.className = "hub-status no-device";
        hubStatusElement.innerText = "No Device";
    }
}

function fillSerialPortMenu(availablePorts, activePort) {
    const serialPortMenu = document.querySelector("#serial-port-menu");
    serialPortMenu.innerHTML = "";

    let optionElement = document.createElement("option");
    optionElement.innerText = "Select port";
    serialPortMenu.appendChild(optionElement);

    availablePorts.forEach((portName) => {
        var optionElement = document.createElement("option");
        optionElement.innerText = portName;

        if (portName === activePort) {
            optionElement.selected = true;
        }

        serialPortMenu.appendChild(optionElement);
    });

}

function setRadioChannel(radioChannel) {
    const radioChannelInput = document.querySelector("#radio-channel")

    if (radioChannel === undefined || radioChannel == "") {
        radioChannelInput.value = "";
        return;
    }

    radioChannelInput.value = radioChannel;
}

function fillPlayersData(newPlayersData) {
    // Get elements to work with
    const playerTable = document.querySelector("#terminal-data-table");
    const tbody = playerTable.childNodes[1];

    // Clear old data
    const oldPlayers = tbody.querySelectorAll(".terminal-data");
    oldPlayers.forEach((oldPlayer) => {
        tbody.removeChild(oldPlayer);
    });

    // Fill with new players
    newPlayersData.forEach((playerData) => {
        let tr = document.createElement("tr");
        tr.className = "terminal-data";
        tbody.appendChild(tr);

        let tdId = document.createElement("td");
        tdId.innerText = playerData.terminalId;
        tr.appendChild(tdId);

        let tdIcon = document.createElement("td");
        tr.appendChild(tdIcon);

        let imagePath = playerData.playerIconPath;
        if ((imagePath === undefined)
            || (imagePath === "")
            || (isImageExisting(imagePath) === false)) {
            console.log("Using default icon for user: " + playerData.playerName);
            imagePath = "./assets/default-icon.png";
        }

        let icon = document.createElement("img");
        icon.src = imagePath;
        icon.className = "player-image";
        tdIcon.appendChild(icon);

        let tdName = document.createElement("td");
        tdName.innerText = playerData.playerName;
        tr.appendChild(tdName);

        let tdUsed = document.createElement("td");
        tr.appendChild(tdUsed);

        let usedCheckBox = document.createElement("input");
        usedCheckBox.type = "checkbox";
        usedCheckBox.checked = playerData.used;
        tdUsed.appendChild(usedCheckBox);
    })
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

function getSettingsConfig() {
    return {
        hubStatus: "Detected",
        hubPort: "/dev/tty3",
        availablePorts: ["/dev/tty1", "/dev/tty2", "/dev/tty3"],
        // radioChannel: "",
        players: [
            {
                terminalId: 35,
                playerIconPath: "./assets/default-icon.png",
                playerName: "Button",
                used: true
            },
            {
                terminalId: 34,
                playerIconPath: "",
                playerName: "Button2",
                used: true
            },
            {
                terminalId: 33,
                playerIconPath: "",
                playerName: "Button3",
                used: true
            },
            {
                terminalId: 32,
                playerIconPath: "",
                playerName: "Button4",
                used: true
            },
            {
                terminalId: 31,
                playerIconPath: "",
                playerName: "Button5",
                used: true
            }
        ]
    }
}

function isImageExisting(imagePath) {
    return false;

    tauri.promisified.fs
        .exists(imagePath)
        .then(exists => {
            if (exists) {
                console.log('Image exists!');
            } else {
                console.log('Image does not exist.');
            }
        })
        .catch(error => {
            console.error('An error occurred while checking image existence:', error);
        });
}