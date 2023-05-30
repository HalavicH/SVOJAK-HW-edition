import { getActivePlayerId, answerQuestion, getQuestionData } from "../service/back-end-com.js";
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

export async function processQustionDisplay(topic, price) {
    console.log("Retreiving question '" + topic + ":" + price + "'");
    const question = await getQuestionData(topic, price);
    console.log("Response"
        + ". questionType: " + question.questionType
        + ", mediaType: " + question.mediaType
        + ", content: " + question.content);

    if (question.questionType === "normal") {
        displayQuestionScreen();
    } else if (question.questionType === "pig-in-poke") {
        processPipPlayers(await getActivePlayerId());
    } else if (question.questionType === "auction") {
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

export async function processCorrectAnswer(event) {
    const player = await answerQuestion(true);
    updateUserScore(player.id);

    // TODO: Clear player selection (.className = "player-badge")
    // TODO: set active player badge
        // 1. getActivePlayerId()
        // 2. .className = "player-badge topic-selection"

    displayRoundScreen();    
}

export async function processWrongAnswer(event) {
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

export async function updateUserScore(playerId) {
    const playerBadges = document.querySelector("#player-list").querySelectorAll(".player-badge");

    playerBadges.forEach((player) => {
        if (playerId === player.querySelector(".player-details-id").innerText) {
            player.querySelector(".player-details-score-value").innerText = response.newScore;
        }
    });
}

