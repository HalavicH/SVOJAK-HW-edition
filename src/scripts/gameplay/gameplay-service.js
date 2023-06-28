const {convertFileSrc} = window.__TAURI__.tauri;

import {
    allowAnswer,
    answerQuestion,
    getActivePlayerId,
    getQuestionData,
    hasNextQuestion,
    isAllowAnswerRequired,
    waitForFirstClick
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

    await processQuestionDisplay(topic, price);

    question.className = "round-td-price used";
}

function placeQuestionContent(question) {
    // Meta data
    document.querySelector("#question-number").innerText = "Question: " + question.number;
    document.querySelector("#question-category").innerText = "Category: " + question.category;
    document.querySelector("#question-price").innerText = "Price: " + question.price;

    const questionViewport = document.querySelector("#question-slider");
    if (questionViewport.currentSlider !== undefined) {
        questionViewport.currentSlider.destroy();
    }
    questionViewport.innerHTML = "";

    const slider = new Slider(question.scenario, questionViewport);
    questionViewport.currentSlider = slider;
    slider.init();
}

class Slider {
    constructor(scenario, questionViewport) {
        this.scenario = scenario;
        this.questionViewport = questionViewport;
        this.currentIndex = 0;
        this.currentSlide = null;
    }

    init() {
        this.showSlide(this.currentIndex);

        const prevButton = document.querySelector("#prev-button");
        const nextButton = document.querySelector("#next-button");

        prevButton.addEventListener("click", this.prevSlide.bind(this));
        nextButton.addEventListener("click", this.nextSlide.bind(this));

        document.addEventListener("keydown", this.handleKeyDown.bind(this));

        if (this.scenario.length <= 1) {
            prevButton.style.display = "none";
            nextButton.style.display = "none";
        } else {
            prevButton.style.display = "unset";
            nextButton.style.display = "unset";
        }
    }

    destroy() {
        console.log("destruction called")
        const prevButton = document.querySelector("#prev-button");
        const nextButton = document.querySelector("#next-button");

        prevButton.removeEventListener("click", this.prevSlide.bind(this));
        nextButton.removeEventListener("click", this.nextSlide.bind(this));

        document.removeEventListener("keydown", this.handleKeyDown.bind(this));
    }

    handleKeyDown(event) {
        if (event.key === "ArrowLeft") {
            this.prevSlide();
        } else if (event.key === "ArrowRight") {
            this.nextSlide();
        } else if (event.key === " ") {
            event.preventDefault(); // Prevent scrolling the page

            if (!this.currentSlide) {
                return;
            }

            if (this.currentSlide.mediaType === "Video") {
                const video = this.currentSlide.element;
                if (video.paused) {
                    video.play();
                } else {
                    video.pause();
                }
            }

            if (this.currentSlide.mediaType === "Voice") {
                const audio = this.currentSlide.element;
                if (audio.paused) {
                    audio.play();
                } else {
                    audio.pause();
                }
            }
        }
    }

    showSlide(index) {
        this.questionViewport.innerHTML = "";

        const slide = this.scenario[index];
        console.log("Processing scenario: " + slide.mediaType + ":" + slide.content);

        if (slide.mediaType === "Say") {
            const text = document.createElement("p");
            text.innerText = slide.content;
            text.className = "question-text";
            this.questionViewport.appendChild(text);
            slide.element = text;
        } else if (slide.mediaType === "Voice") {
            const audio = document.createElement("audio");
            audio.src = convertFileSrc(slide.content);
            audio.controls = true;
            audio.className = "question-audio";
            this.questionViewport.appendChild(audio);
            slide.element = audio;
        } else if (slide.mediaType === "Video") {
            const video = document.createElement("video");
            video.src = convertFileSrc(slide.content);
            video.controls = true;
            video.className = "question-video";
            this.questionViewport.appendChild(video);
            slide.element = video;
        } else if (slide.mediaType === "Image") {
            const image = document.createElement("img");
            image.src = convertFileSrc(slide.content);
            image.alt = "Question Image";
            image.className = "question-image";
            this.questionViewport.appendChild(image);
            slide.element = image;
        } else {
            console.log("Not supported format");

            // Display content as text anyway
            const element = document.createElement("p");
            element.innerText = slide.content;
            element.className = "question-text";
            this.questionViewport.appendChild(element);
            slide.element = element;
        }

        this.currentSlide = slide; // Update the current slide reference
    }

    prevSlide() {
        this.currentIndex--;
        if (this.currentIndex < 0) {
            this.currentIndex = this.scenario.length - 1;
        }
        this.showSlide(this.currentIndex);
    }

    nextSlide() {
        this.currentIndex++;
        if (this.currentIndex >= this.scenario.length) {
            this.currentIndex = 0;
        }
        this.showSlide(this.currentIndex);
    }
}


async function setAnswerButtonsAccordingToQuestionType() {
    // Disable allow button
    const button = document.querySelector("#allow-answer-btn");
    if (await (isAllowAnswerRequired()) === true) {
        button.style.display = "block";
        document.querySelector("#correct-answer-btn").className = "inactive";
        document.querySelector("#wrong-answer-btn").className = "inactive";
    } else {
        button.style.display = "none";
        document.querySelector("#correct-answer-btn").className = "";
        document.querySelector("#wrong-answer-btn").className = "";
    }
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

    await setAnswerButtonsAccordingToQuestionType();

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
    await answerQuestion(true);

    updatePlayers();
    goToRoundScreen();
}

export async function processWrongAnswer() {
    await setAnswerButtonsAccordingToQuestionType();

    let retry = await answerQuestion(false);
    updatePlayers();

    if (!retry) {
        goToRoundScreen();
    }
}

export async function goToRoundScreen() {
    updatePlayers();
    if (await hasNextQuestion()) {
        displayRoundScreen();
    } else {
        showRoundStats();
    }
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