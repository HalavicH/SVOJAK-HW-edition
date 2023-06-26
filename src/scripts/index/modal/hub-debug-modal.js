import {openModal, closeModal} from "../../service/modal-common.js";
import {getSettingsConfig} from "../../service/back-end-com.js";

const {invoke} = window.__TAURI__.tauri;
const {open} = window.__TAURI__.dialog;

export function setupHubDebugCallbacks() {
    document
        .querySelector("#open-hub-debug")
        .addEventListener("click", openHubDebugModal);

    document
        .querySelector("#close-hub-debug-modal")
        .addEventListener("click", closeHubDebugModal);

    document
        .querySelector("#send-request-btn")
        .addEventListener("click", sendRawHubRequest);

    document
        .querySelector("#dev-serial-port-menu")
        .addEventListener("change", serialPortSelectHandler);
}

async function openHubDebugModal() {
    console.log("opened");
    const modal = document.querySelector("#hub-debug-modal");

    const config = await getSettingsConfig();

    // if (config.hub_port !== "") {
    //     discoverHubAndSetStatus(config.hub_port);
    // }
    //
    fillSerialPortMenu(config.available_ports, config.hub_port);
    openModal(modal);
}

function fillSerialPortMenu(availablePorts, activePort) {
    const serialPortMenu = document.querySelector("#dev-serial-port-menu");
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

export async function serialPortSelectHandler(event) {
    // Get the selected option value
    const selectedOption = event.target.value;

    // Perform actions based on the selected option
    console.log("Selected option:", selectedOption);
    invoke("setup_hub_connection", {portName: selectedOption})
        .then(setPortStatus)
        .catch(setPortStatus);
}

function setPortStatus(status) {
    const portStatusElement = document.querySelector("#port-status-field");
    console.log("Port status received: " + status);

    if (status === null) {
        portStatusElement.className = "hub-status detected";
        portStatusElement.innerText = "Port Opened";
    } else if (status === "SerialPortError") {
        portStatusElement.className = "hub-status serial-port-error";
        portStatusElement.innerText = "Serial port error";
    } else {
        portStatusElement.className = "hub-status serial-port-error";
        portStatusElement.innerText = "Internal Error";
    }
}
async function closeHubDebugModal() {
    console.log("closing");
    const modalPackInfoContainer = document.querySelector("#hub-debug-modal");

    closeModal(modalPackInfoContainer);
}

function setOkStatus() {
    let status = document.querySelector("#request-status-value");
    status.textContent = "Operation successful";
    status.className = "request-status ok";
}

function setErrorStatus(statusText) {
    let status = document.querySelector("#request-status-value");
    status.textContent = statusText;
    status.className = "request-status";
}

async function sendRawHubRequest() {
    const input = document.querySelector("#request-from-input");
    let innerText = input.value;

    if (innerText.trim().length === 0) {
        console.log("Empty request. Doing nothing");
        return;
    }

    let frame = [];
    let inputTokens = innerText.trim().split(" ");
    inputTokens.forEach(token => {
        if (token.length !== 2) {
            console.log("Byte '" + token + "' is not two characters like 'XX'")
            setErrorStatus("Invalid input");
            throw new Error("Invalid input aborting...");
        }

        let number = parseInt(token, 16);
        if (isNaN(number)) {
            console.log("Byte '" + token + "' is not valid HEX");
            setErrorStatus("Invalid input");
            throw new Error("Invalid input aborting...");
        }
        frame.push(number);
    })

    console.log("Parsed request frame as dec: " + frame);
    invoke("send_raw_request_frame", {requestFrame: frame})
        .then(responseFrame => {
            setOkStatus();
            console.log("Response frame as dec: " + responseFrame);

            const hexString = responseFrame
                .map(decimalValue => decimalValue.toString(16).padStart(2, '0').toUpperCase())
                .join(" ");
            console.log("Response frame as HEX string: " + hexString);

            const output = document.querySelector("#response-content-value");
            output.innerText = hexString;
        })
        .catch(err => {
            console.error("Can't process request: " + err);
            setErrorStatus(err);
        });
}
