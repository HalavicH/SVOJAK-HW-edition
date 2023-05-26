import { fetchPlayers, fetchRound } from "./back-end-com.js";
import { getImagePathOrDefault } from "./utils.js";

console.log("Gameplay loaded!");

window.addEventListener("DOMContentLoaded", () => {
    // modal processing
    document.querySelector("#go-to-main-menu").addEventListener("click", () => {
        window.location.href = "./index.html";
    });

    // document.querySelector("#go-to-main-menu").addEventListener("click", () => {
    //     window.location.href = "./index.html";
    // });

    processPlayers();
    processRoundFromBackend();
});

// Todo: видалити старих гравців, та додати нових
export function processPlayers() {
    const players = fetchPlayers();
    const playerList = document.querySelector("#player-list");
    playerList.innerHTML = "";

    players.forEach((player) => {
        let playerBadge = document.createElement("div");
        playerBadge.className = "player-badge";
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

    });
}

function processRoundFromBackend() {
    const round = fetchRound();
    const packList = document.querySelector("#round-data-tbody")
    packList.innerHTML = "";

    document.querySelector("#round-label").innerText = "Round: " + round.roundName;

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
        let tdPrice1 = document.createElement("td");
        tdPrice1.className = "round-td-price";
        tdPrice1.innerText = topic.questions.price1;
        tdPrice1.appendChild(topicMarker.cloneNode(true));
        tdPrice1.addEventListener("click", processQuestionSelection);
        tr.appendChild(tdPrice1);

        let tdPrice2 = document.createElement("td");
        tdPrice2.className = "round-td-price";
        tdPrice2.innerText = topic.questions.price2;
        tdPrice2.appendChild(topicMarker.cloneNode(true));
        tdPrice2.addEventListener("click", processQuestionSelection);
        tr.appendChild(tdPrice2);

        let tdPrice3 = document.createElement("td");
        tdPrice3.className = "round-td-price";
        tdPrice3.innerText = topic.questions.price3;
        tdPrice3.appendChild(topicMarker.cloneNode(true));
        tdPrice3.addEventListener("click", processQuestionSelection);
        tr.appendChild(tdPrice3);

        let tdPrice4 = document.createElement("td");
        tdPrice4.className = "round-td-price";
        tdPrice4.innerText = topic.questions.price4;
        tdPrice4.appendChild(topicMarker.cloneNode(true));
        tdPrice4.addEventListener("click", processQuestionSelection);
        tr.appendChild(tdPrice4);

        let tdPrice5 = document.createElement("td");
        tdPrice5.className = "round-td-price";
        tdPrice5.innerText = topic.questions.price5;
        tdPrice5.appendChild(topicMarker.cloneNode(true));
        tdPrice5.addEventListener("click", processQuestionSelection);
        tr.appendChild(tdPrice5);
    });
} 

function processQuestionSelection(event) {
    const question = event.target;
    const price = question.innerText;
    const topic = question.querySelector("div").innerText;

    console.log("Triggered");
    console.log("Data: ", event.target);
    console.log("Retreiving question '" + topic + ":" + price + "'");

    question.className = "round-td-price used";
}