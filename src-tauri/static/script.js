// Views
let registerView = document.querySelector('#register');
let waitGameView = document.querySelector('#wait-for-game');
let gameView = document.querySelector('#content');

// Buttons
let submitPlayer = document.querySelector('#submit-player');
let answerButton = document.querySelector('#answer');

// Inputs
let playerNameInput = document.querySelector('#name');

// Info
let baseTimestampDiv = document.querySelector('#base-timestamp');
let playerIdDiv = document.querySelector('#player-id');
let playerNameDiv = document.querySelector('#player-name');
let playerStatusDiv = document.querySelector('#player-status');

STATE = {
    playerId: undefined,
    baseTimestamp: undefined,
}

async function registerPlayer() {
    const playerName = playerNameInput.value;

    if (playerName === "") {
        console.log("Empty input");
        return;
    }

    console.log("Name: " + playerName);

    let body = JSON.stringify({
        id: 0,
        name: playerName,
        ip: "0.0.0.0",
    });
    const response = await fetch("/register", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: body,
    });
    if (response.ok) {
        const {playerId, baseTimestamp} = await response.json();
        // Store the playerId and baseTimestamp in your application as needed
        STATE.playerId = playerId;
        STATE.baseTimestamp = baseTimestamp;

        console.log("Player state: " + STATE);

        baseTimestampDiv.innerText = baseTimestamp;
        playerIdDiv.innerText = playerId;
        playerNameDiv.innerText = playerName;

        // Disable the register screen and enable the content screen
        registerView.style.display = "none";
        gameView.style.display = "flex";
    } else {
        console.error("Failed to register player");
    }
}

async function sendEvent(buttonState) {
    const playerId = STATE.playerId; // Replace with the actual playerId
    const eventData = {
        id: playerId,
        state: buttonState,
        timestamp: STATE.baseTimestamp,
    };

    const response = await fetch("/event", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(eventData),
    });

    if (!response.ok) {
        console.error("Failed to send event");
        return;
    }

    response.json()
        .then(value => {
            console.log(value);

            playerStatusDiv.style.color = value.color;
        })
}


document.addEventListener("DOMContentLoaded", () => {
    waitGameView.style.display = 'none';
    gameView.style.display = 'none';

    submitPlayer.addEventListener("click", registerPlayer);
    answerButton.addEventListener("mousedown", () => {
        sendEvent(true); // Button is pressed
    });
    answerButton.addEventListener("mouseup", () => {
        sendEvent(false); // Button is pressed
    });
})

