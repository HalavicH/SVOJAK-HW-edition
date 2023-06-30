import {openModal, closeModal} from "../../service/modal-common.js";
import {
    getSettingsConfig,
    savePlayers,
    probeHub,
    discoverPlayers,
    setHubRadioChannel, setHubType
} from "../../service/back-end-com.js";
import {getImagePathOrDefault} from "../../service/utils.js";
import {setupHubDebugCallbacks} from "./hub-debug-modal.js";

const {invoke} = window.__TAURI__.tauri;

// Global
let settingsModal = document.querySelector("#settings-modal");
let hwHubSettingsModal = document.querySelector("#hw-hub-settings-modal");
let webHubSettingsModal = document.querySelector("#web-hub-settings-modal");

// Tables
const serialTerminalTable = hwHubSettingsModal.querySelector("#terminal-data-table");
const webPlayerTable = webHubSettingsModal.querySelector("#players-data-table");

// Status
const serialHubStatusDiv = hwHubSettingsModal.querySelector("#hub-status-field");
const webHubStatusDiv = webHubSettingsModal.querySelector("#web-hub-status-field");

function commonSettingsCallbacks() {
    document
        .querySelector("#settings-button")
        .addEventListener("click", openSettingsModal);

    document
        .querySelector("#close-settings-modal")
        .addEventListener("click", closeSettingsModal);
}

function hwSettingsCallbacks() {
    document
        .querySelector("#hw-hub-btn")
        .addEventListener("click", openHwHubSettingsModal);

    document
        .querySelector("#close-hw-hub-settings-modal")
        .addEventListener("click", closeHwHubSettingsModal);

    document
        .querySelector("#save-hw-hub-settings-modal")
        .addEventListener("click", saveHwHubSettingsModal);

    document
        .querySelector("#set-hub-radio-channel")
        .addEventListener("click", handleSetHubRadioChannel);

    document
        .querySelector("#refresh-terminals-btn")
        .addEventListener("click", async () => {
            await handleDiscoverTerminals(serialTerminalTable);
        });

    document
        .querySelector("#serial-port-menu")
        .addEventListener("change", serialPortSelectHandler);
}

function webSettingsCallbacks() {
    document
        .querySelector("#web-hub-btn")
        .addEventListener("click", openWebHubSettingsModal);

    document
        .querySelector("#close-web-hub-settings-modal")
        .addEventListener("click", closeWebHubSettingsModal);

    document
        .querySelector("#save-web-hub-settings-modal")
        .addEventListener("click", saveWebHubSettingsModal);

    document
        .querySelector("#refresh-web-players-btn")
        .addEventListener("click", async () => {
            await handleDiscoverTerminals(webPlayerTable);
        });
}

// SETUP //
export function setupSettingsModalCallbacks() {
    commonSettingsCallbacks();
    hwSettingsCallbacks();
    webSettingsCallbacks();

    setupHubDebugCallbacks();
}

// Settings modal
function openSettingsModal() {
    openModal(settingsModal);
}

function closeSettingsModal() {
    closeModal(settingsModal);
}

// Hw HUB settings //
export async function openHwHubSettingsModal() {
    await setHubType("HwHub");
    closeModal(settingsModal);
    openModal(hwHubSettingsModal);

    const config = await getSettingsConfig();

    if (config.hub_port !== "") {
        discoverHubAndSetStatus(config.hub_port, serialHubStatusDiv);
    }

    fillSerialPortMenu(config.available_ports, config.hub_port);
    setRadioChannel(config.radio_channel);
    const playerTable = hwHubSettingsModal.querySelector("#terminal-data-table");
    fillPlayersData(config.players, playerTable);
}

export function closeHwHubSettingsModal() {
    closeModal(hwHubSettingsModal);
}

export function saveHwHubSettingsModal() {
    processPlayerDataSaving(serialTerminalTable);

    closeModal(hwHubSettingsModal);
}

function processPlayerDataSaving(playersTable) {
    let playerElementList = playersTable.querySelectorAll(".terminal-data");

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

function setHubStatus(status, hubStatusElement) {
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
    const serialPortMenu = hwHubSettingsModal.querySelector("#serial-port-menu");
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
    const radioChannelInput = hwHubSettingsModal.querySelector("#radio-channel");

    if (radioChannel === undefined || radioChannel === 0) {
        radioChannelInput.value = "";
        return;
    }

    radioChannelInput.value = radioChannel;
}

function fillPlayersData(newPlayersData, playerTable) {
    // Get elements to work with
    const tbody = playerTable.childNodes[1];

    // Clear old data
    const oldPlayers= tbody.querySelectorAll(".terminal-data");
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

export async function handleDiscoverTerminals(playersTable) {
    const terminals = await discoverPlayers();

    console.info("result = " + terminals);

    fillPlayersData(terminals, playersTable);
}

function discoverHubAndSetStatus(selectedOption, hubStatusDiv) {
    probeHub(selectedOption)
        .then((status => setHubStatus(status, hubStatusDiv)))
        .catch((err => setHubStatus(err, hubStatusDiv)))
}

export async function serialPortSelectHandler(event) {
    // Get the selected option value
    const selectedOption = event.target.value;

    // Perform actions based on the selected option
    console.log("Selected option:", selectedOption);
    discoverHubAndSetStatus(selectedOption, serialHubStatusDiv);
    await handleDiscoverTerminals(serialTerminalTable);
}

export async function handleSetHubRadioChannel() {
    console.log("Set radio channel...");
    const channelIdObject = hwHubSettingsModal.querySelector("#radio-channel");

    await setHubRadioChannel(channelIdObject.value);
    await handleDiscoverTerminals(serialTerminalTable)
}

// WEB HUB settings //
async function openWebHubSettingsModal() {
    await setHubType("WebHub");

    closeModal(settingsModal);
    openModal(webHubSettingsModal);

    const config = await getSettingsConfig();

    discoverHubAndSetStatus(config.hub_port, serialHubStatusDiv);

    discoverHubAndSetStatus("Nothing", webHubStatusDiv);
    let hubIpDiv = webHubSettingsModal.querySelector("#hub-ip-field");

    hubIpDiv.innerText = config.hub_port;
    // TODO: Set player polling
}

function closeWebHubSettingsModal() {
    closeModal(webHubSettingsModal);
}

function saveWebHubSettingsModal() {
    processPlayerDataSaving(webPlayerTable);

    closeModal(webHubSettingsModal);
}