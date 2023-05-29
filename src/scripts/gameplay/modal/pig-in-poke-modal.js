import { fetchPlayers, sendPipVictim } from "../../service/back-end-com.js";
import { closeModal, openModal } from "../../service/modal-common.js";
import { getImagePathOrDefault } from "../../service/utils.js";
import { displayQuestionScreen } from "../gameplay-service.js";


// Todo: видалити старих гравців, та додати нових
export async function processPipPlayers(activePlayerId) {
    const players = await fetchPlayers();
    const playerList = document.querySelector("#player-victim-list");
    playerList.innerHTML = "";

    players.forEach((player) => {
        // TODO: Compare termId
        if (player.id === activePlayerId) {
            console.log("Player with id: '" + activePlayerId + "' removed from pip because he is choosing the victim");
            return;
        }
        
        let playerBadge = document.createElement("div");
        playerBadge.className = "player-victim-badge";
        playerBadge.addEventListener("click", processVictimSelection);
        playerBadge.style.cursor = "pointer";
        playerList.appendChild(playerBadge);
        // playerBadge.click();

        let playerIcon = document.createElement("div");
        playerIcon.className = "player-icon";
        playerBadge.appendChild(playerIcon);

        let icon = document.createElement("img");
        icon.src = getImagePathOrDefault(player.playerIconPath);
        playerIcon.appendChild(icon);

        let playerName = document.createElement("p");
        playerName.className = "name";
        playerName.innerText = player.playerName;
        playerBadge.appendChild(playerName);
    });

    const modal = document.querySelector("#pig-in-poke-modal");

    openModal(modal);
}

async function processVictimSelection(event) { 
    // Єбаний костиль.
    const victim = event.target.parentNode.parentNode;
    const name = victim.querySelector("p").innerText;
    console.log("Victim is: " + name);
    
    sendPipVictim(name);

    const modal = document.querySelector("#pig-in-poke-modal");
    closeModal(modal);

    displayQuestionScreen();
}
