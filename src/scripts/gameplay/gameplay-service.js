const { convertFileSrc } = window.__TAURI__.tauri;

import {
    allowAnswer,
    answerQuestion,
    finishQuestionPrematurely,
    getActivePlayerId,
    getQuestionData,
    hasNextQuestion,
    isAllowAnswerRequired,
    waitForFirstClick,
} from "../service/back-end-com.js";
import { processPipPlayers } from "./modal/pig-in-poke-modal.js";
import { processAuctionPlayers } from "./modal/auction-modal.js";
import { showRoundStats } from "./modal/round-stats-modal.js";
import { displayPlayers } from "./setup.js";

let qCtx = {
    price: undefined,
    category: undefined,
    answer: undefined,
    slider: undefined,
};

const REFS = {
    // Div //
    questionNumberDiv: document.querySelector("#question-number"),
    questionCategoryDiv: document.querySelector("#question-category"),
    questionPriceDiv: document.querySelector("#question-price"),

    // Buttons //
    prevSlideBtn: document.querySelector("#prev-button"),
    nextButton: document.querySelector("#next-button"),
    allowAnswerBtn: document.querySelector("#allow-answer-btn"),
    correctAnswerBtn: document.querySelector("#correct-answer-btn"),
    wrongAnswerBtn: document.querySelector("#wrong-answer-btn"),

    // Screens //
    roundViewport: document.querySelector("#round-screen"),
    questionViewport: document.querySelector("#question-screen"),

    // //Badges
    playerBadges: document.querySelector("#player-list").querySelectorAll(".player-badge"),

    // List //
    playerList: document.querySelector("#player-list"),
};

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
    REFS.questionNumberDiv.innerText = "Question: " + question.number;
    REFS.questionCategoryDiv.innerText = "Category: " + question.category;
    REFS.questionPriceDiv.innerText = "Price: " + question.price;
    REFS.questionPriceDiv.innerText = "Answer: " + question.answer;

    const questionViewport = document.querySelector("#question-slider");
    if (questionViewport.currentSlider !== undefined) {
        questionViewport.currentSlider.destroy();
    }
    questionViewport.innerHTML = "";

    const slider = new Slider(question.scenario, questionViewport, question.answer);
    questionViewport.currentSlider = slider;
    slider.init();
    qCtx.slider = slider;
}

class Slider {
    constructor(scenario, questionViewport, answer) {
        this.scenario = scenario;
        this.questionViewport = questionViewport;
        this.answer = answer;
        this.currentIndex = 0;
        this.currentSlide = null;
    }

    init() {
        this.showSlide(this.currentIndex);
        REFS.prevSlideBtn.addEventListener("click", this.prevSlide.bind(this));
        REFS.nextButton.addEventListener("click", this.nextSlide.bind(this));

        document.addEventListener("keydown", this.handleKeyDown.bind(this));

        if (this.scenario.length <= 1) {
            REFS.prevSlideBtn.style.display = "none";
            REFS.nextButton.style.display = "none";
        } else {
            REFS.prevSlideBtn.style.display = "unset";
            REFS.nextButton.style.display = "unset";
        }
    }

    showAnswer() {
        this.questionViewport.innerHTML = "";

        const text = document.createElement("p");
        text.innerText = this.answer;
        text.className = "question-text";

        let colDiv = document.createElement("div");
        colDiv.style.display = "flex";
        colDiv.style.flexDirection = "column";
        colDiv.style.justifyContent = "center";
        colDiv.style.alignItems = "center";
        this.questionViewport.appendChild(colDiv);
        colDiv.appendChild(text);
        let toRound = document.createElement("button");
        toRound.addEventListener("click", displayRoundScreen);
        toRound.innerText = "Go to round screen";
        colDiv.appendChild(toRound);
    }

    destroy() {
        console.log("destruction called");
        REFS.prevSlideBtn.removeEventListener("click", this.prevSlide.bind(this));
        REFS.nextButton.removeEventListener("click", this.nextSlide.bind(this));

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
    if ((await isAllowAnswerRequired()) === true) {
        REFS.allowAnswerBtn.style.display = "block";
        REFS.correctAnswerBtn.className = "inactive";
        REFS.wrongAnswerBtn.className = "inactive";
    } else {
        button.style.display = "none";
        REFS.correctAnswerBtn.className = "";
        REFS.wrongAnswerBtn.className = "";
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

    qCtx.price = question.price;
    qCtx.category = question.category;
    qCtx.answer = question.answer;

    await setAnswerButtonsAccordingToQuestionType();

    placeQuestionContent(question);

    if (question.questionType === "Normal") {
        displayQuestionScreen();
    } else if (question.questionType === "PigInPoke") {
        processPipPlayers(await getActivePlayerId());
    } else if (question.questionType === "Auction") {
        processAuctionPlayers(await getActivePlayerId());
    }
    updatePlayers();
}

export function displayQuestionScreen() {
    // Disable round viewport
    REFS.roundViewport.style.display = "none";

    // Enable question viewport
    REFS.questionViewport.style.display = "flex";
}

export function displayRoundScreen() {
    // Disable question viewport
    REFS.questionViewport.style.display = "none";

    // Enable round viewport
    REFS.roundViewport.style.display = "flex";
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
    REFS.correctAnswerBtn.className = "";
    REFS.wrongAnswerBtn.className = "";
}

export function updateUserScore(responcePlayer) {
    REFS.playerBadges.forEach((player) => {
        if (responcePlayer.id == player.querySelector(".player-details-id").innerText) {
            player.querySelector(".player-details-score-value").innerText = responcePlayer.newScore;
        }
    });
}

export function setAllPlayersState(state) {
    const playerBadges = REFS.playerList.querySelectorAll(".player-badge");

    playerBadges.forEach((player) => {
        player.className = "player-badge " + state;
    });
}

export async function setActivePlayerBadgeState(state) {
    const activePlayer = await getActivePlayerId();
    const playerBadges = REFS.playerList.querySelectorAll(".player-badge");

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

function showAnswerModal() {
    qCtx.slider.showAnswer();
}

export async function processShowAnswer() {
    finishQuestionPrematurely()
        .then(() => {
            console.log("Answer: " + qCtx.answer);
            showAnswerModal();
        })
        .catch((err) => {
            console.error("Can't finish question: " + err);
            showAnswerModal();
            // displayRoundScreen();
        });
}
