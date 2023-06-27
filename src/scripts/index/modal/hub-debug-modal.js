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

    document
        .querySelector("#dev-hub-command-menu")
        .addEventListener("change", onCommandMenuChange);

    document.querySelector("#send-command-btn")
        .addEventListener("click", createRequest);

}

function onCommandMenuChange() {
    const commandMenu = document.querySelector("#dev-hub-command-menu");
    const parameter1Input = document.querySelector("#request-parameter-1");
    const parameter2Input = document.querySelector("#request-parameter-2");
    // Get the selected option value
    const selectedOption = commandMenu.value;

    setUndefinedCommandStatus();
    // Perform actions based on the selected option
    switch (selectedOption) {
        case "set_timestamp":
            parameter1Input.style.visibility = "visible";
            parameter2Input.style.visibility = "hidden";
            parameter1Input.placeholder = "Timestamp (0xDEADBEEF)";
            break;
        case "get_timestamp":
            parameter1Input.style.visibility = "hidden";
            parameter2Input.style.visibility = "hidden";
            break;
        case "set_hub_radio_channel":
            parameter1Input.style.visibility = "visible";
            parameter2Input.style.visibility = "hidden";
            parameter1Input.placeholder = "Channel (0x06)";
            break;
        case "set_term_radio_channel":
            parameter1Input.style.visibility = "visible";
            parameter2Input.style.visibility = "visible";

            parameter1Input.placeholder = "Term ID (0x06)";
            parameter2Input.placeholder = "Channel (0x06)";
            break;
        case "ping_device":
            parameter1Input.style.visibility = "visible";
            parameter2Input.style.visibility = "hidden";
            parameter1Input.placeholder = "Term ID (0x06)";
            break;
        case "set_light_color":
            parameter1Input.style.visibility = "visible";
            parameter2Input.style.visibility = "visible";
            parameter1Input.placeholder = "Term ID (0x06)";
            parameter2Input.placeholder = "Color RGB (0xFFAABB)";
            break;
        case "set_feedback_led":
            parameter1Input.style.visibility = "visible";
            parameter2Input.style.visibility = "visible";
            parameter1Input.placeholder = "Term ID (0x06)";
            parameter2Input.placeholder = "State (0x1/0x0)";
            break;
        case "read_event_queue":
            parameter1Input.style.visibility = "hidden";
            parameter2Input.style.visibility = "hidden";
            break;
        default:
            parameter1Input.style.visibility = "hidden";
            parameter2Input.style.visibility = "hidden";
            break;
    }
}

function createRequest() {
    const commandMenu = document.querySelector("#dev-hub-command-menu");
    const parameter1Input = document.querySelector("#request-parameter-1");
    const parameter2Input = document.querySelector("#request-parameter-2");

    // Get the selected option value and parameter values
    const selectedOption = commandMenu.value;
    const param1Hex = parameter1Input.value.trim();
    const param2Hex = parameter2Input.value.trim();

    const shouldHaveParameters = (selectedOption !== "get_timestamp") && (selectedOption !== "read_event_queue");
    if (shouldHaveParameters && param1Hex === "" && param2Hex === "") {
        console.log("No input. Do nothing");
        return;
    }

    let param1IntVal, param2IntVal;

    switch (selectedOption) {
        case "set_timestamp":
            if (param1Hex === "") {
                console.log("Timestamp input required");
                setErrorCommandStatus("Invalid input: Timestamp required");
                return;
            }
            if (param1Hex.length > 10 || !param1Hex.startsWith("0x")) {
                console.log("Invalid timestamp input: " + param1Hex);
                setErrorCommandStatus("Invalid input: Invalid timestamp");
                return;
            }
            param1IntVal = parseInt(param1Hex, 16);
            if (isNaN(param1IntVal)) {
                console.log("Invalid timestamp input: " + param1IntVal);
                setErrorCommandStatus("Invalid input: Invalid timestamp");
                return;
            }
            break;

        case "get_timestamp":
            // No parameters required
            break;

        case "set_hub_radio_channel":
            if (param1Hex === "") {
                console.log("Channel input required");
                setErrorCommandStatus("Invalid input: Channel required");
                return;
            }
            if (param1Hex.length > 4 || !param1Hex.startsWith("0x")) {
                console.log("Invalid channel input: " + param1Hex);
                setErrorCommandStatus("Invalid input: Invalid channel");
                return;
            }
            param1IntVal = parseInt(param1Hex, 16);
            if (isNaN(param1IntVal)) {
                console.log("Invalid channel input: " + param1IntVal);
                setErrorCommandStatus("Invalid input: Invalid channel");
                return;
            }
            break;

        case "set_term_radio_channel":
            if (param1Hex === "" || param2Hex === "") {
                console.log("Term ID and Channel inputs required");
                setErrorCommandStatus("Invalid input: Term ID and Channel required");
                return;
            }
            if (param1Hex.length > 4 || !param1Hex.startsWith("0x")) {
                console.log("Invalid Term ID input: " + param1Hex);
                setErrorCommandStatus("Invalid input: Invalid Term ID");
                return;
            }
            if (param2Hex.length > 4 || !param2Hex.startsWith("0x")) {
                console.log("Invalid Channel input: " + param2Hex);
                setErrorCommandStatus("Invalid input: Invalid Channel");
                return;
            }
            param1IntVal = parseInt(param1Hex, 16);
            param2IntVal = parseInt(param2Hex, 16);
            if (isNaN(param1IntVal) || isNaN(param2IntVal)) {
                console.log("Invalid Term ID or Channel input: " + param1IntVal + ", " + param2IntVal);
                setErrorCommandStatus("Invalid input: Invalid Term ID or Channel");
                return;
            }
            break;

        case "ping_device":
            if (param1Hex === "") {
                console.log("Term ID input required");
                setErrorCommandStatus("Invalid input: Term ID required");
                return;
            }
            if (param1Hex.length > 4 || !param1Hex.startsWith("0x")) {
                console.log("Invalid Term ID input: " + param1Hex);
                setErrorCommandStatus("Invalid input: Invalid Term ID");
                return;
            }
            param1IntVal = parseInt(param1Hex, 16);
            if (isNaN(param1IntVal)) {
                console.log("Invalid Term ID input: " + param1IntVal);
                setErrorCommandStatus("Invalid input: Invalid Term ID");
                return;
            }
            break;

        case "set_light_color":
            if (param1Hex === "" || param2Hex === "") {
                console.log("Term ID and Color inputs required");
                setErrorCommandStatus("Invalid input: Term ID and Color required");
                return;
            }
            if (param1Hex.length > 4 || !param1Hex.startsWith("0x")) {
                console.log("Invalid Term ID input: " + param1Hex);
                setErrorCommandStatus("Invalid input: Invalid Term ID");
                return;
            }
            if (param2Hex.length > 8 || !param2Hex.startsWith("0x")) {
                console.log("Invalid Color input: " + param2Hex);
                setErrorCommandStatus("Invalid input: Invalid Color");
                return;
            }
            param1IntVal = parseInt(param1Hex, 16);
            param2IntVal = parseInt(param2Hex, 16);
            if (isNaN(param1IntVal) || isNaN(param2IntVal)) {
                console.log("Invalid Term ID or Color input: " + param1IntVal + ", " + param2IntVal);
                setErrorCommandStatus("Invalid input: Invalid Term ID or Color");
                return;
            }
            break;

        case "set_feedback_led":
            if (param1Hex === "" || param2Hex === "") {
                console.log("Term ID and State inputs required");
                setErrorCommandStatus("Invalid input: Term ID and State required");
                return;
            }
            if (param1Hex.length > 4 || !param1Hex.startsWith("0x")) {
                console.log("Invalid Term ID input: " + param1Hex);
                setErrorCommandStatus("Invalid input: Invalid Term ID");
                return;
            }
            if (param2Hex.length > 4 || !param2Hex.startsWith("0x")) {
                console.log("Invalid State input: " + param2Hex);
                setErrorCommandStatus("Invalid input: Invalid State");
                return;
            }
            param1IntVal = parseInt(param1Hex, 16);
            param2IntVal = parseInt(param2Hex, 16);
            if (isNaN(param1IntVal) || isNaN(param2IntVal)) {
                console.log("Invalid Term ID or State input: " + param1IntVal + ", " + param2IntVal);
                setErrorCommandStatus("Invalid input: Invalid Term ID or State");
                return;
            }
            break;

        case "read_event_queue":
            // No parameters required
            break;

        default:
            console.log("Invalid command");
            setErrorCommandStatus("Invalid command");
            return;
    }

    // Create the request object
    const request = {
        cmd: selectedOption,
        param1: isNaN(param1IntVal) ? 0 : param1IntVal,
        param2: isNaN(param2IntVal) ? 0 : param2IntVal,
    };

    // Perform further actions with the request object
    console.log(request); // Example: Log the request object to the console
    invoke("send_hub_command", {request: request})
        .then(response => {
            setOkCommandStatus();
            console.log("Response object: ", response);

            // Fill response fields
            const requestFrame = document.querySelector("#request-frame");
            const responseFrame = document.querySelector("#response-frame");
            const responseObject = document.querySelector("#response-object");

            requestFrame.textContent = response.request_frame;
            responseFrame.textContent = response.response_frame;
            responseObject.innerHTML = `<pre>${response.response_obj}</pre>`;
        })
        .catch(err => {
            console.error("Can't process request: " + err);
            setErrorCommandStatus(err);
        });
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

function setOkCommandStatus() {
    let status = document.querySelector("#command-status-value");
    status.textContent = "Operation successful";
    status.className = "request-status ok";
}

function setUndefinedCommandStatus() {
    let status = document.querySelector("#command-status-value");
    status.textContent = "Undefined";
    status.className = "request-status undefined";
}

function setErrorCommandStatus(statusText) {
    let status = document.querySelector("#command-status-value");
    status.textContent = statusText;
    status.className = "request-status";
}

function toHexList(responseFrame) {
    const hexString = responseFrame
        .map(decimalValue => decimalValue.toString(16).padStart(2, '0').toUpperCase())
        .join(" ");
    return hexString;
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
            const hexString = toHexList(responseFrame);
            console.log("Response frame as HEX string: " + hexString);

            const output = document.querySelector("#response-content-value");
            output.innerText = hexString;
        })
        .catch(err => {
            console.error("Can't process request: " + err);
            setErrorStatus(err);
        });
}
