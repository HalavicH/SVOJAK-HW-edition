import {
    getActivePlayerId,
    answerQuestion,
    getQuestionData,
    allowAnswer,
    waitForFirstClick,
    isAllowAnswerRequired,
    hasNextQuestion,
    fetchPlayers
} from "../service/back-end-com.js";
import {processPipPlayers} from "./modal/pig-in-poke-modal.js";
import {processAuctionPlayers} from "./modal/auction-modal.js";
import {showRoundStats} from "./modal/round-stats-modal.js";
import {displayPlayers} from "./setup.js";

export async function processQuestionSelection(event) {
    const question = event.target;
    const price = question.innerText;
    const topic = question.querySelector("div").innerText;

    console.info("Question selected: " + topic + ":" + price);

    if (question.className.includes("used")) {
        return;
    }

    question.className = "round-td-price used";

    await processQuestionDisplay(topic, price);
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
    text.innerText = question.scenario[0].content;
    text.className = "question-text";
    questionViewport.appendChild(text);
}

export async function processQuestionDisplay(topic, price) {
    console.log("Retreiving question '" + topic + ":" + price + "'");
    const question = await getQuestionData(topic, parseInt(price));
    console.log(
        "Response" +
        ". questionType: " +
        question.questionType +
        ", mediaType: " +
        question.scenario[0].mediaType +
        ", content: " +
        question.scenario[0].content
    );

    // Disable allow button
    const button = document.querySelector("#allow-answer-btn");
    if (await (isAllowAnswerRequired()) == true) {
        button.style.display = "block";
        document.querySelector("#correct-answer-btn").className = "inactive";
        document.querySelector("#wrong-answer-btn").className = "inactive";
    } else {
        button.style.display = "none";
        document.querySelector("#correct-answer-btn").className = "";
        document.querySelector("#wrong-answer-btn").className = "";
    }

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
    console.log("Correct answer pressed");
    const player = await answerQuestion(true);
    updateUserScore(player);

    // await setActivePlayerBadgeState("wrong-answer");
    updatePlayers();

    goToRoundScreen();
}

export async function goToRoundScreen() {
    setAllPlayersState("");

    if (await hasNextQuestion()) {
        // setActivePlayerBadgeState("topic-selection");
        updatePlayers();
        displayRoundScreen();
    } else {
        showRoundStats();
    }
}

export async function processWrongAnswer() {
    console.log("Activeeeee!!!!!!!!!!!!!");

    const player = await answerQuestion(false);
    updateUserScore(player.id);

    updatePlayers();
    // await setActivePlayerBadgeState("wrong-answer");
    // TODO: set active player badge
    // 1. getActivePlayerId()
    // 2. .className = "player-badge inactive"
    // 3. forbidAnswer()
    // 4. listen

    // ifNot
    // if question not nornal:
    // return
    // TODO: make player inactive, set answer forbidden
}

export async function allowAnswerHandler() {
    allowAnswer();
    await waitForFirstClick();
    updatePlayers();
    // setActivePlayerBadgeState("first-response");
    document.querySelector("#correct-answer-btn").className = "";
    document.querySelector("#wrong-answer-btn").className = "";
}

export function updateUserScore(responcePlayer) {
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

export function updatePlayers() {
    console.log("Updating players view");
    displayPlayers();
}