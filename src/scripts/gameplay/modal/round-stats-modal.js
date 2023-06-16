import {fetchRoundStats, initNextRound} from "../../service/back-end-com.js";
import {closeModal, openModal} from "../../service/modal-common.js";
import {getImagePathOrDefault} from "../../service/utils.js";
import {displayRoundScreen} from "../gameplay-service.js";
import {loadRoundFromBackend} from "../setup.js";

export function showRoundStats() {
    const modalContainer = document.querySelector("#stats-modal");
    openModal(modalContainer);

    updateWithNewRoundStats()
}

export function updateWithNewRoundStats() {
    fetchRoundStats()
        .then(stats => {
            setRoundNumber(stats.roundName);
            setTotalQuestion(stats.questionNumber);
            setNormalQuestions(stats.normalQuestionNum);
            setPipQuestions(stats.pigInPokeQuestionNum);
            setTotalCorrect(stats.totalCorrectAnswers);
            setTotalWrong(stats.totalWrongAnswers);
            // setTotalTries(stats.totalTries);
            setRoundTime(stats.roundTime);
            fillPlayersStats(stats.players);
        });
}

function fillPlayersStats(playerStats) {
    const roundStatsTbody = document.querySelector("#player-stats-table")
        .querySelector("tbody");
    const statsLabel = roundStatsTbody.querySelector(".dark-table-labels");
    roundStatsTbody.innerHTML = "";
    roundStatsTbody.appendChild(statsLabel);

    playerStats.forEach((stats) => {
        let tr = document.createElement("tr");
        roundStatsTbody.appendChild(tr);

        let tdIcon = document.createElement("td");
        tr.appendChild(tdIcon);

        let icon = document.createElement("img");
        icon.src = getImagePathOrDefault(stats.playerIconPath);
        icon.className = "player-image";
        tdIcon.appendChild(icon);

        let name = document.createElement("td");
        name.innerText = stats.name;
        tr.appendChild(name);

        let score = document.createElement("td");
        score.innerText = stats.score;
        tr.appendChild(score);

        let correct = document.createElement("td");
        correct.innerText = stats.answeredCorrectly;
        tr.appendChild(correct);

        let wrong = document.createElement("td");
        wrong.innerText = stats.answeredWrong;
        tr.appendChild(wrong);

        let total = document.createElement("td");
        total.innerText = stats.totalAnswers;
        tr.appendChild(total);

    });
}

function setTotalWrong(wrong) {
    const totalWrongAnswers = document.querySelector("#wrong-answers");
    totalWrongAnswers.innerText = "Total wrong answers: " + wrong;
}

function setRoundTime(time) {
    const roundTime = document.querySelector("#round-time");
    roundTime.innerText = "Round time: " + time;
}

function setTotalCorrect(correct) {
    const totalCorrectAnswers = document.querySelector("#correct-answers");
    totalCorrectAnswers.innerText = "Total correct answers: " + correct;
}

function setPipQuestions(pip) {
    const pigQuestionsElement = document.querySelector("#pip-question");
    pigQuestionsElement.innerText = "Pig in poke questions: " + pip;
}

function setNormalQuestions(normal) {
    const normalQuestionsElement = document.querySelector("#normal-question");
    normalQuestionsElement.innerText = "Normal: " + normal;
}

function setRoundNumber(number) {
    const roundNumberElement = document.querySelector("#round-number");
    roundNumberElement.innerText = "Round: " + number;
}

function setTotalQuestion(total) {
    const totalQuestionElement = document.querySelector("#total-questions");
    totalQuestionElement.innerText = "Total questions: " + total;
}

export function nextRoundHandler() {
    const modalContainer = document.querySelector("#stats-modal");
    closeModal(modalContainer);
    displayRoundScreen();
    initNextRound()
        .then(async () => {
            await loadRoundFromBackend();
        })
}


