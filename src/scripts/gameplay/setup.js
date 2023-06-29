import {fetchPlayers, fetchRound} from "../service/back-end-com.js";
import {closeModal, openModal} from "../service/modal-common.js";
import {getImagePathOrDefault} from "../service/utils.js";
import {
    processCorrectAnswer,
    processWrongAnswer,
    processQuestionSelection,
    allowAnswerHandler
} from "./gameplay-service.js";
import {nextRoundHandler} from "./modal/round-stats-modal.js";

console.log("Gameplay loaded!");

// SETUP //
window.addEventListener("DOMContentLoaded", () => {
    addButtonEventListeners();
    displayPlayers();
    loadRoundFromBackend();
});

function addButtonEventListeners() {
    // modal processing
    document.querySelectorAll(".go-to-main-menu")
        .forEach((button) => {
            console.log("Applying handler to button: " + button);
            button.addEventListener("click", () => {
                const modal = document.querySelector("#exit-dialog-modal");
                openModal(modal);

            });
        });
    document
        .querySelector("#correct-answer-btn")
        .addEventListener("click", processCorrectAnswer);

    document
        .querySelector("#wrong-answer-btn")
        .addEventListener("click", processWrongAnswer);

    document
        .querySelector("#allow-answer-btn")
        .addEventListener("click", allowAnswerHandler);

    document
        .querySelector("#exit-dialog-yes")
        .addEventListener("click", () => {
            window.location.href = "./index.html";
        });

    document
        .querySelector("#exit-dialog-no")
        .addEventListener("click", closeExitDialogModal);

    document
        .querySelector("#next-round-btn")
        .addEventListener("click", nextRoundHandler);
}

function closeExitDialogModal() {
    const modal = document.querySelector("#exit-dialog-modal");

    closeModal(modal);
}

export async function displayPlayers() {
    const players = await fetchPlayers();
    const playerList = document.querySelector("#player-list");
    playerList.innerHTML = "";

    players.forEach((player) => {
        addMainScreenPlayer(player, playerList)
    });
}

function mapPlayerStateToClass(state) {
    if (state === "Idle") {
        return "";
    }
    if (state === "QuestionChooser") {
        return "question-chooser";
    }
    if (state === "Target") {
        return "target-player";
    }
    if (state === "FirstResponse") {
        return "first-response";
    }
    if (state === "Inactive") {
        return "inactive";
    }
    if (state === "Dead") {
        return "game-over";
    }
    if (state === "AnsweredCorrectly") {
        return "correct-answer";
    }
    if (state === "AnsweredWrong") {
        return "wrong-answer";
    }
}

function addMainScreenPlayer(player, playerList) {
    let stateClass = mapPlayerStateToClass(player.state);

    let playerBadge = document.createElement("div");
    playerBadge.className = "player-badge " + stateClass;
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

    let playerDetailsId = document.createElement("div");
    playerDetailsId.className = "player-details-id";
    playerDetailsId.style.display = "none";
    playerDetailsId.innerText = player.id;
    playersDetails.appendChild(playerDetailsId);

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
}

export async function loadRoundFromBackend() {
    const round = await fetchRound();
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
        let questionWidth = 65 / topic.questions.length;
        topic.questions.forEach((question) => {
            addQuestion(question.price, topicMarker, tr, questionWidth);
        })
    });
}

function addQuestion(price, marker, tr, questionWidth) {
    let tdQuestion = document.createElement("td");
    tdQuestion.className = "round-td-price";
    tdQuestion.style.width = `${questionWidth}%`;
    tdQuestion.innerText = price;
    tdQuestion.appendChild(marker.cloneNode(true));
    tdQuestion.addEventListener("click", processQuestionSelection);
    tr.appendChild(tdQuestion);
}
