import { getActivePlayerId, answerQuestion, getQuestionData, allowAnswer, waitForFirstClick } from "../service/back-end-com.js";
import { processPipPlayers } from "./modal/pig-in-poke-modal.js";
import { processAuctionPlayers } from "./modal/auction-modal.js";

export async function processQuestionSelection(event) {
    const question = event.target;
    const price = question.innerText;
    const topic = question.querySelector("div").innerText;

    console.info("Question selected: " + topic + ":" + price);

    if (question.className.includes("used")) {
        return;
    }

    question.className = "round-td-price used";

    await processQustionDisplay(topic, price);
}

function placeQuestionContent(question) {
    // Meta data
    document.querySelector("#question-number").innerText = "Question: " + question.number;
    document.querySelector("#question-category").innerText = "Category: " + question.category;
    document.querySelector("#question-price").innerText = "Price: " + question.price;

    // TODO: Add proper media handling
    const questionViewport = document.querySelector(".question-viewport");
    questionViewport.innerHTML = "";

    const text = document.createElement("p");
    text.innerText = question.content;
    text.className = "question-text";
    questionViewport.appendChild(text);
}

export async function processQustionDisplay(topic, price) {
    console.log("Retreiving question '" + topic + ":" + price + "'");
    const question = await getQuestionData(topic, parseInt(price));
    console.log(
        "Response" +
            ". questionType: " +
            question.questionType +
            ", mediaType: " +
            question.mediaType +
            ", content: " +
            question.content
    );

    placeQuestionContent(question);

    if (question.questionType === "Normal") {
        displayQuestionScreen();
    } else if (question.questionType === "PigInPoke") {
        processPipPlayers(await getActivePlayerId());
    } else if (question.questionType === "Auction") {
        processAuctionPlayers(await getActivePlayerId());
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

export function displayRoundScreen() {
    // Disable question viewport
    const questionViewport = document.querySelector("#question-screen");
    questionViewport.style.display = "none";

    // Enable round viewport
    const roundViewport = document.querySelector("#round-screen");
    roundViewport.style.display = "flex";
}

export async function processCorrectAnswer() {
    const player = await answerQuestion(true);
    updateUserScore(player);
    goToRoundScreen();
}

export function goToRoundScreen() {
    setAllPlayersState("");
    setActivePlayerBadgeState("topic-selection");
    displayRoundScreen();
}

export async function processWrongAnswer() {
    console.log("Activeeeee!!!!!!!!!!!!!");

    const player = await answerQuestion(false);
    updateUserScore(player.id);

    // TODO: set active player badge
    // 1. getActivePlayerId()
    // 2. .className = "player-badge inactive"
    // 3. forbidAnswer()
    // 4. listen

    // TODO: make player inactive, set answer forbidden
}

export async function allowAnswerHandler() {
    allowAnswer();
    await waitForFirstClick();
    setActivePlayerBadgeState("first-response");
}

export async function updateUserScore(responcePlayer) {
    const playerBadges = document.querySelector("#player-list").querySelectorAll(".player-badge");

    playerBadges.forEach((player) => {
        if (responcePlayer.id == player.querySelector(".player-details-id").innerText) {
            player.querySelector(".player-details-score-value").innerText = responcePlayer.newScore;
        }
    });
}

export function setAllPlayersState(state) {
    const playerList = document.querySelector("#player-list");
    const playerBadges = playerList.querySelectorAll(".player-badge");

    playerBadges.forEach((player) => {
        player.className = "player-badge " + state;
    });
}

export async function setActivePlayerBadgeState(state) {
    const activePlayer = await getActivePlayerId();
    const playerList = document.querySelector("#player-list");
    const playerBadges = playerList.querySelectorAll(".player-badge");

    playerBadges.forEach((player) => {
        const id = player.querySelector(".player-details-id").innerText;
        if (id == activePlayer) {
            player.className = "player-badge " + state;
        }
    });
}
