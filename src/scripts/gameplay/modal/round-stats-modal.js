import { fetchRoundStats, initNextRound } from "../../service/back-end-com.js";
import { closeModal, openModal } from "../../service/modal-common.js";
import { getImagePathOrDefault } from "../../service/utils.js";
import { displayRoundScreen } from "../gameplay-service.js";
import { loadRoundFromBackend } from "../setup.js";

const REFS = {
    // Common //
    statsModalDiv: document.querySelector("#stats-modal"),
    roundStatsTbody: document.querySelector("#player-stats-table").querySelector("tbody"),

    // Li //
    roundTimeLi: document.querySelector("#round-time"),
    totalWrongAnswersLi: document.querySelector("#wrong-answers"),
    totalCorrectAnswersLi: document.querySelector("#correct-answers"),
    pigQuestionsElementLi: document.querySelector("#pip-question"),
    normalQuestionsElementLi: document.querySelector("#normal-question"),
    roundNumberElementLi: document.querySelector("#round-number"),
    totalQuestionElementLi: document.querySelector("#total-questions"),
};

export function showRoundStats() {
    openModal(REFS.statsModalDiv);

    updateWithNewRoundStats();
}

export function updateWithNewRoundStats() {
    fetchRoundStats().then((stats) => {
        REFS.roundTimeLi.innerText = "Round time: " + stats.roundTime;
        REFS.roundNumberElementLi.innerText = "Round: " + stats.roundName;

        REFS.normalQuestionsElementLi.innerText = "Normal: " + stats.normalQuestionNum;
        REFS.pigQuestionsElementLi.innerText = "Pig in poke questions: " + stats.pigInPokeQuestionNum;

        REFS.totalWrongAnswersLi.innerText = "Total wrong answers: " + stats.totalWrongAnswers;
        REFS.totalCorrectAnswersLi.innerText = "Total correct answers: " + stats.totalCorrectAnswers;
        REFS.totalQuestionElementLi.innerText = "Total questions: " + stats.questionNumber;

        fillPlayersStats(stats.players);
    });
}

function fillPlayersStats(playerStats) {
    const statsLabel = REFS.roundStatsTbody.querySelector(".dark-table-labels");
    REFS.roundStatsTbody.innerHTML = "";
    REFS.roundStatsTbody.appendChild(statsLabel);

    playerStats.forEach((stats) => {
        let tr = document.createElement("tr");
        REFS.roundStatsTbody.appendChild(tr);

        tr.innerHTML = `
            <td>
                <img src=${getImagePathOrDefault(stats.playerIconPath)} class="player-image"/>            
            </td>
            <td>${stats.name}</td>
            <td>${stats.score}</td>
            <td>${stats.answeredCorrectly}</td>
            <td>${stats.answeredWrong}</td>
            <td>${stats.totalAnswers}</td>
            `;
        REFS.roundStatsTbody.appendChild(tr);
    });
}

export function nextRoundHandler() {
    closeModal(REFS.statsModalDiv);
    displayRoundScreen();
    initNextRound().then(async () => {
        await loadRoundFromBackend();
    });
}
