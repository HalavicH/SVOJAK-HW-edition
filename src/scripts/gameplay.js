import { fetchPlayers, fetchRound, getActivePlayer, getUpdatedScore, getQuestionData } from "./back-end-com.js";
import { processPipPlayers } from "./pig-in-poke-modal.js";
import { getImagePathOrDefault } from "./utils.js";

console.log("Gameplay loaded!");

window.addEventListener("DOMContentLoaded", () => {
    // modal processing
    document.querySelectorAll(".go-to-main-menu")
        .forEach((button) => {
            console.log("Applying handler to button: " + button);
            button.addEventListener("click", () => {
                window.location.href = "./index.html";
        });
    });
    
    document
        .querySelector("#correct-answer-btn")
        .addEventListener("click", processCorrectAnswer);

    processMainScreenPlayers();
    processRoundFromBackend();
});

// Todo: видалити старих гравців, та додати нових
export async function processMainScreenPlayers() {
    const players = await fetchPlayers();
    const playerList = document.querySelector("#player-list");
    playerList.innerHTML = "";

    players.forEach((player) => {
        let playerBadge = document.createElement("div");
        playerBadge.className = "player-badge";
        playerList.appendChild(playerBadge);

        let playerIcon = document.createElement("div");
        playerIcon.className = "player-icon";
        playerBadge.appendChild(playerIcon);

        let icon = document.createElement("img");
        icon.src = getImagePathOrDefault(player.playerIconPath);
        playerIcon.appendChild(icon);

        let playersDetails = document.createElement("div");
        playersDetails.className = "player-details";
        playerBadge.appendChild(playersDetails);

        let playerDetailsName = document.createElement("p");
        playerDetailsName.className = "player-details-name";
        playerDetailsName.innerText = player.playerName;
        playersDetails.appendChild(playerDetailsName);

        let playerDetailsScore = document.createElement("div");
        playerDetailsScore.className = "player-details-score";
        playerDetailsScore.innerText = "Score: ";
        playersDetails.appendChild(playerDetailsScore);

        let score = document.createElement("p");
        score.className = "player-details-score-value";
        score.innerText = player.score;
        playerDetailsScore.appendChild(score);

    });
}

function processRoundFromBackend() {
    const round = fetchRound();
    const packList = document.querySelector("#round-data-tbody")
    packList.innerHTML = "";

    document.querySelectorAll(".round-label")
        .forEach((label) => {
            label.innerText = "Round: " + round.roundName;
        });

    round.roundTopics.forEach((topic) => {
        // Create row
        let tr = document.createElement("tr");
        packList.appendChild(tr);

        // Create topic marker
        let topicMarker = document.createElement("div");
        topicMarker.style.display = "none";
        topicMarker.innerText = topic.topicName;

        // Create topic name
        let tdTheme = document.createElement("td");
        tdTheme.className = "round-td-theme";
        tdTheme.innerText = topic.topicName;
        tr.appendChild(tdTheme);

        ////////// 1-5 questions //////////
        addQuestion(topic.questions.price1, topicMarker, tr);
        addQuestion(topic.questions.price2, topicMarker, tr);
        addQuestion(topic.questions.price3, topicMarker, tr);
        addQuestion(topic.questions.price4, topicMarker, tr);
        addQuestion(topic.questions.price5, topicMarker, tr);        
    });
} 

function addQuestion(price, marker, tr) {
    let tdQuestion = document.createElement("td");
    tdQuestion.className = "round-td-price";
    tdQuestion.innerText = price;
    tdQuestion.appendChild(marker.cloneNode(true));
    tdQuestion.addEventListener("click", processQuestionSelection);
    tr.appendChild(tdQuestion);
}

async function processQuestionSelection(event) {
    const question = event.target;
    const price = question.innerText;
    const topic = question.querySelector("div").innerText;

    console.info("Triggered");

    if (question.className.includes("used")) {
        return;
    }

    question.className = "round-td-price used";
    await processQustionDisplay(topic, price);
}

async function processQustionDisplay(topic, price) {
    console.log("Retreiving question '" + topic + ":" + price + "'");
    const question = await getQuestionData(topic, price);
    console.log("Response"
        + ". questionType: " + question.questionType
        + ", mediaType: " + question.mediaType
        + ", content: " + question.content);

    if (question.questionType === "normal") {
        displayQuestionScreen();
    } else if (question.questionType === "pig-in-poke") {
        processPipPlayers(await getActivePlayer());
    } else if (question.questionType === "auction") {
        // TODO: schow auction modal
    }
}

export function displayQuestionScreen() {
    // Disable round viewport
    const roundViewport = document.querySelector("#round-screen");
    roundViewport.style.display = "none";

    // Enable question viewport
    const questionViewport = document.querySelector("#question-screen");
    questionViewport.style.display = "flex";
}

function displayRoundScreen() {
    // Disable question viewport
    const questionViewport = document.querySelector("#question-screen");
    questionViewport.style.display = "none";
    
    // Enable round viewport
    const roundViewport = document.querySelector("#round-screen");
    roundViewport.style.display = "flex";
}

function processCorrectAnswer(event) {
    displayRoundScreen();
    
    updateUserScore(true);
}

async function updateUserScore(isCorrect) {
    const response = await getUpdatedScore(isCorrect);
    console.log("Response is: name: " + response.targetPlayer + " score: " + response.newScore);

    const playerBadges = document.querySelector("#player-list").querySelectorAll(".player-badge");

    playerBadges.forEach((player) => {
        if (response.targetPlayer === player.querySelector(".player-details-name").innerText) {
            player.querySelector(".player-details-score-value").innerText = response.newScore;
        }
    });
}

// <div class="player-details">
//     <p class="player-details-name">HalavicH</p>
//     <p class="player-details-score">Score: 100</p>
// </div>
