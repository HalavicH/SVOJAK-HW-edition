
import {openModal, closeModal} from "./modal-common.js";
import {getSettingsConfig} from "./../back-end-com.js";
import {isImageExisting} from "../utils.js";


const {invoke} = window.__TAURI__.tauri;

export async function openSettingsModal() {
    const modalContainer = document.querySelector("#settings-modal");
    openModal(modalContainer);
    const config = getSettingsConfig();

    setHubStatus(config.hubStatus);
    fillSerialPortMenu(config.availablePorts, config.hubPort);
    setRadioChannel(config.radioChannel);
    fillPlayersData(config.players);

}

export function closeSettingsModal() {
    const modalContainer = document.querySelector("#settings-modal");

    closeModal(modalContainer);
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

export async function discover() {
    const channelIdObject = document.querySelector("#radio-channel");
    const result = await invoke("discover_terminals", {
        channelId: parseInt(channelIdObject.value),
    });

    console.info("result = " + result);
}

export async function serialPortSelectHandler(event) {
    // Get the selected option value
    const selectedOption = event.target.value;

    // Perform actions based on the selected option
    console.log("Selected option:", selectedOption);

    const result = await invoke("open_selected_port", { path: selectedOption });

    console.info("serialPortSelectHandler: result = " + result);

    setHubStatus(result);
}



