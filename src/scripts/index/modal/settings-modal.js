import { openModal, closeModal } from "../../service/modal-common.js";
import {
    getSettingsConfig,
    savePlayers,
    probeHub,
    discoverPlayers,
    setHubRadioChannel,
    setHubType,
} from "../../service/back-end-com.js";
import { getImagePathOrDefault } from "../../service/utils.js";
import { setupHubDebugCallbacks } from "./hub-debug-modal.js";

const { invoke } = window.__TAURI__.tauri;

const REFS = {
    /////////// COMMON ///////////
    settingsModal: document.querySelector("#settings-modal"),

    // Buttons
    openModalBtn: document.querySelector("#settings-button"),
    closeModalBtn: document.querySelector("#close-settings-modal"),

    ///////////   HW   ///////////
    hwHubSettingsModal: document.querySelector("#hw-hub-settings-modal"),
    // Tables
    hwTerminalDataTable: document.querySelector("#terminal-data-table"),

    // Buttons
    hwOpenModalBtn: document.querySelector("#hw-hub-btn"),
    hwCloseModalBtn: document.querySelector("#close-hw-hub-settings-modal"),
    hwSaveSettingsBtn: document.querySelector("#save-hw-hub-settings-modal"),
    hwSetRcBtn: document.querySelector("#set-hub-radio-channel"),
    hwRefreshTermsBtn: document.querySelector("#refresh-terminals-btn"),

    // Inputs
    hwRcInput: document.querySelector("#radio-channel"),
    hwSerialPortMenu: document.querySelector("#serial-port-menu"),

    // Divs
    hwHubStatusDiv: document.querySelector("#hub-status-field"),

    ///////////   WEB  ///////////
    webHubSettingsModal: document.querySelector("#web-hub-settings-modal"),

    // Tables
    webTerminalDataTable: document.querySelector("#players-data-table"),

    // Buttons
    webOpenModalBtn: document.querySelector("#web-hub-btn"),
    webCloseModalBtn: document.querySelector("#close-web-hub-settings-modal"),
    webSaveModalBtn: document.querySelector("#save-web-hub-settings-modal"),
    webRefreshTermsBtn: document.querySelector("#refresh-web-players-btn"),

    // Divs
    webHubStatusDiv: document.querySelector("#web-hub-status-field"),
    webHubIpDiv: document.querySelector("#hub-ip-field"),
};

function commonSettingsCallbacks() {
    REFS.openModalBtn.addEventListener("click", openSettingsModal);
    REFS.closeModalBtn.addEventListener("click", closeSettingsModal);
}

function hwSettingsCallbacks() {
    REFS.hwOpenModalBtn.addEventListener("click", openHwHubSettingsModal);
    REFS.hwCloseModalBtn.addEventListener("click", closeHwHubSettingsModal);
    REFS.hwSaveSettingsBtn.addEventListener("click", saveHwHubSettingsModal);
    REFS.hwSetRcBtn.addEventListener("click", handleSetHubRadioChannel);
    REFS.hwRefreshTermsBtn.addEventListener("click", async () => {
        await handleDiscoverTerminals(REFS.hwTerminalDataTable);
    });

    REFS.hwSerialPortMenu.addEventListener("change", serialPortSelectHandler);
}

function webSettingsCallbacks() {
    REFS.webOpenModalBtn.addEventListener("click", openWebHubSettingsModal);
    REFS.webCloseModalBtn.addEventListener("click", closeWebHubSettingsModal);
    REFS.webSaveModalBtn.addEventListener("click", saveWebHubSettingsModal);
    REFS.webRefreshTermsBtn.addEventListener("click", async () => {
        await handleDiscoverTerminals(REFS.webTerminalDataTable);
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
    openModal(REFS.settingsModal);
}

function closeSettingsModal() {
    closeModal(REFS.settingsModal);
}

// Hw HUB settings //
export async function openHwHubSettingsModal() {
    await setHubType("HwHub");
    closeModal(REFS.settingsModal);
    openModal(REFS.hwHubSettingsModal);

    const config = await getSettingsConfig();

    if (config.hub_port !== "") {
        discoverHubAndSetStatus(config.hub_port, REFS.hwHubStatusDiv);
    }

    fillSerialPortMenu(config.available_ports, config.hub_port);
    setRadioChannel(config.radio_channel);
    const playerTable = REFS.hwTerminalDataTable;
    fillPlayersData(config.players, playerTable);
}

export function closeHwHubSettingsModal() {
    closeModal(REFS.hwHubSettingsModal);
}

export function saveHwHubSettingsModal() {
    processPlayerDataSaving(REFS.hwTerminalDataTable);

    closeModal(REFS.hwHubSettingsModal);
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
    REFS.hwSerialPortMenu.innerHTML = "";

    let optionElement = document.createElement("option");
    optionElement.innerText = "Select port";
    REFS.hwSerialPortMenu.appendChild(optionElement);

    availablePorts.forEach((portName) => {
        var optionElement = document.createElement("option");
        optionElement.innerText = portName;

        if (portName === activePort) {
            optionElement.selected = true;
        }

        REFS.hwSerialPortMenu.appendChild(optionElement);
    });
}

function setRadioChannel(radioChannelNum) {
    if (radioChannelNum === undefined || radioChannelNum === 0) {
        REFS.hwRcInput.value = "";
        return;
    }

    REFS.hwRcInput.value = radioChannelNum;
}

function fillPlayersData(newPlayersData, playerTable) {
    // Get elements to work with
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

export async function handleDiscoverTerminals(playersTable) {
    const terminals = await discoverPlayers();

    console.info("result = " + terminals);

    fillPlayersData(terminals, playersTable);
}

function discoverHubAndSetStatus(selectedOption, hubStatusDiv) {
    probeHub(selectedOption)
        .then((status) => setHubStatus(status, hubStatusDiv))
        .catch((err) => setHubStatus(err, hubStatusDiv));
}

export async function serialPortSelectHandler(event) {
    // Get the selected option value
    const selectedOption = event.target.value;

    // Perform actions based on the selected option
    console.log("Selected option:", selectedOption);
    discoverHubAndSetStatus(selectedOption, REFS.hwHubStatusDiv);
    await handleDiscoverTerminals(REFS.hwTerminalDataTable);
}

export async function handleSetHubRadioChannel() {
    console.log("Set radio channel...");
    const channelIdObject = REFS.hwRcInput;

    await setHubRadioChannel(channelIdObject.value);
    await handleDiscoverTerminals(REFS.hwTerminalDataTable);
}

// WEB HUB settings //
async function openWebHubSettingsModal() {
    await setHubType("WebHub");

    closeModal(REFS.settingsModal);
    openModal(REFS.webHubSettingsModal);

    const config = await getSettingsConfig();

    discoverHubAndSetStatus(config.hub_port, REFS.hwHubStatusDiv);

    discoverHubAndSetStatus("Nothing", REFS.webHubStatusDiv);

    REFS.webHubIpDiv.innerText = config.hub_port;
    // TODO: Set player polling

    setInterval(queryWebPlayers, 1000);
}

function closeWebHubSettingsModal() {
    closeModal(REFS.webHubSettingsModal);
}

function saveWebHubSettingsModal() {
    processPlayerDataSaving(REFS.webTerminalDataTable);

    closeModal(REFS.webHubSettingsModal);
}

function modalIsOpen() {
    return REFS.webHubSettingsModal.style !== "none";
}

// Start the interval
let intervalId = undefined;

async function queryWebPlayers() {
    if (!modalIsOpen()) {
        console.log("Modal is closed! Clearing the interval");
        clearInterval(intervalId);
    } else {
        await handleDiscoverTerminals(REFS.webTerminalDataTable);
    }
}
