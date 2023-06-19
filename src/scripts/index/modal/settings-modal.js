import {openModal, closeModal} from "../../service/modal-common.js";
import {getSettingsConfig, savePlayers, probeHub, discoverTerminals} from "../../service/back-end-com.js";
import {getImagePathOrDefault} from "../../service/utils.js";

const {invoke} = window.__TAURI__.tauri;

export async function openSettingsModal() {
    const modalContainer = document.querySelector("#settings-modal");
    openModal(modalContainer);
    const config = await getSettingsConfig();

    if (config.hub_port !== "") {
        discoverHubAndSetStatus(config.hub_port);
    }

    fillSerialPortMenu(config.available_ports, config.hub_port);
    setRadioChannel(config.radio_channel);
    fillPlayersData(config.players);
}

export function closeSettingsModal() {
    const modalContainer = document.querySelector("#settings-modal");

    processPlayerDataSaving();

    closeModal(modalContainer);
}

function processPlayerDataSaving() {
    let playerElementList = document.querySelector("#terminal-data-table").querySelectorAll(".terminal-data");

    let playerDataList = [];

    playerElementList.forEach((playerRow) => {
        const playerDataElements = playerRow.querySelectorAll("td");

        const id = parseInt(playerDataElements[0].innerText);
        const icon = "./assets/game-over-picture.png"; // TODO: save icon from playerDataElements[1];
        const name = playerDataElements[2].firstChild.value;
        const used = playerDataElements[3].firstChild.checked;

        const playerData = {
            termId: id,
            icon: icon,
            name: name,
            isUsed: used,
            score: 0,
        };

        playerDataList.push(playerData);
    });

    // TODO: Save player list
    savePlayers(playerDataList);
}

function setHubStatus(status) {
    const hubStatusElement = document.querySelector("#hub-status-field");
    console.log("Hub status received: " + status);

    if (status === "Detected") {
        hubStatusElement.className = "hub-status detected";
        hubStatusElement.innerText = "Hub Detected";
    } else if (status === "SerialPortError") {
        hubStatusElement.className = "hub-status serial-port-error";
        hubStatusElement.innerText = "Serial port error";
    } else if (status === "NoResponseFromHub" || status === "NoDevice") {
        hubStatusElement.className = "hub-status unknown-device";
        hubStatusElement.innerText = "Unknown Device";
    } else {
        hubStatusElement.className = "hub-status serial-port-error";
        hubStatusElement.innerText = "Internal Error";
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
    const radioChannelInput = document.querySelector("#radio-channel");

    if (radioChannel === undefined || radioChannel === 0) {
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
        tdId.innerText = playerData.termId;
        tr.appendChild(tdId);

        let tdIcon = document.createElement("td");
        tr.appendChild(tdIcon);

        const imagePath = getImagePathOrDefault(playerData.icon);

        let icon = document.createElement("img");
        icon.src = imagePath;
        icon.className = "player-image";
        tdIcon.appendChild(icon);

        // <input class="term-name" placeholder="Enter player name" type="text"></input>
        let tdName = document.createElement("td");
        let input = document.createElement("input");
        input.className = "term-name";
        input.placeholder = "Enter player name";
        input.type = "text";
        input.value = playerData.name;
        tdName.appendChild(input);
        tr.appendChild(tdName);

        let tdUsed = document.createElement("td");
        tr.appendChild(tdUsed);

        let usedCheckBox = document.createElement("input");
        usedCheckBox.type = "checkbox";
        usedCheckBox.checked = playerData.isUsed;
        tdUsed.appendChild(usedCheckBox);
    });
}


export async function handleDiscoverTerminals() {
    const channelIdObject = document.querySelector("#radio-channel");
    const terminals = await discoverTerminals(channelIdObject);

    console.info("result = " + terminals);

    let mockPlayers = [];
    terminals.forEach((id) => {
        mockPlayers.push({
            termId: id,
            icon: "",
            name: "",
            isUsed: true
        });
    });

    fillPlayersData(mockPlayers);
}

function discoverHubAndSetStatus(selectedOption) {
    probeHub(selectedOption)
        .then(setHubStatus)
        .catch(setHubStatus);
}

export async function serialPortSelectHandler(event) {
    // Get the selected option value
    const selectedOption = event.target.value;

    // Perform actions based on the selected option
    console.log("Selected option:", selectedOption);
    discoverHubAndSetStatus(selectedOption);
}

