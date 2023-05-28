import { openModal, closeModal } from "./modal-common.js";
import { getPackInfo, saveRoundDuration } from "./../back-end-com.js";

const {invoke} = window.__TAURI__.tauri;

export async function openPackInfoModal() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");

    const filePath = "";
    const packInfo = getPackInfo(filePath);

    setPackName(packInfo.packName);
    setPackAuthor(packInfo.packAuthor);
    setPackRounds(packInfo.packRounds);
    setPackTopics(packInfo.packTopics);
    setPackQuestion(packInfo.packQuestion);
    setPackTopicList(packInfo.packTopicList);

    openModal(modalPackInfoContainer);
}

function setPackTopicList(packTopicList) {
    const packTopicListElement = document.querySelector("#topic-list");
    packTopicListElement.innerHTML = "";

    packTopicList.forEach((topic) => {
        let li = document.createElement("li");
        li.innerText = topic;
        packTopicListElement.appendChild(li);
    });
}

function setPackQuestion(packQuestion) {
    const packQuestionElement = document.querySelector("#pack-question-num");
    packQuestionElement.innerText = "Question: " + packQuestion;
}

function setPackTopics(packTopics) {
    const packTopicsElement = document.querySelector("#pack-topics-num");
    packTopicsElement.innerText = "Topics: " + packTopics;
}

function setPackRounds(packRounds) {
    const packRoundsElement = document.querySelector("#pack-round-num");
    packRoundsElement.innerText = "Rounds: " + packRounds;
}

function setPackName(packName) {
    const packNameElement = document.querySelector("#pack-name");
    packNameElement.innerText = "Pack: " + packName;
}

function setPackAuthor(packAuthor) {
    const packAuthorElement = document.querySelector("#pack-author-form");
    packAuthorElement.innerText = "Author: " + packAuthor;
}

export function closePackInfoModal() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");

    closeModal(modalPackInfoContainer);
}

export function startTheGame() {
    const raundDurationOptions = document
    .querySelector("#round-duration")
    .querySelectorAll("option");
    let duration = 0;
    raundDurationOptions.forEach((option) => {
        if (option.selected) {
            duration = parseInt(option.value) 
        }
    });

    saveRoundDuration(duration);

    window.location.href = "./gameplay.html";
}


