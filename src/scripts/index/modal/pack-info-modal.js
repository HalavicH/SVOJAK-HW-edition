import {openModal, closeModal} from "../../service/modal-common.js";
import {getPackInfo, saveRoundDuration, startTheGame} from "../../service/back-end-com.js";

const {invoke} = window.__TAURI__.tauri;
const {open} = window.__TAURI__.dialog;

export async function openPackInfoModal() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");

    const filePath = await open({
        multiple: false,
        filters: [{
            name: 'Select game package',
            extensions: ['siq']
        }]
    });

    if (filePath === null) {
        console.error("Game package file wasn't selected");
        return;
    } else {
        console.info("Selected game package path: ", filePath);
    }

    await getPackInfo(filePath)
        .then((packInfo) => {
            setPackName(packInfo.packName);
            setPackAuthor(packInfo.packAuthor);
            setPackRounds(packInfo.packRounds);
            setPackTopics(packInfo.packTopics);
            setPackQuestions(packInfo.packQuestions);
            setPackTopicList(packInfo.packTopicList);

            openModal(modalPackInfoContainer);
        })
        .catch((error) => {
            console.error("Promise rejection:", error);
            // Log the rejection payload or handle the error in any other way
            openPackErrorModel(error);
        });
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

function setPackQuestions(packQuestion) {
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

export function handleStartTheGame() {
    const roundDurationOptions = document
        .querySelector("#round-duration")
        .querySelectorAll("option");

    let duration = 0;
    roundDurationOptions.forEach((option) => {
        if (option.selected) {
            duration = parseInt(option.value)
        }
    });

    saveRoundDuration(duration);

    startTheGame();
    window.location.href = "./gameplay.html";
}

function openPackErrorModel(error) {
    let errorModel = document.querySelector("#pack-error-modal");
    errorModel.querySelector("#pack-path").innerText = error.path;
    errorModel.querySelector("#pack-error-cause").innerText = error.cause;
    errorModel.querySelector("#pack-error-details").innerHTML = error.details.replaceAll("\n", "<br>");

    openModal(errorModel);
}

export async function closePackErrorModal() {
    let modal = document.querySelector("#pack-error-modal");
    closeModal(modal);
}
