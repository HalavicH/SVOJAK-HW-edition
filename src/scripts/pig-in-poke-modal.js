import { fetchPlayers, sendPipVictim } from "./back-end-com.js";
import { closeModal } from "./modal/modal-common.js";
import { getImagePathOrDefault } from "./utils.js";


// Todo: видалити старих гравців, та додати нових
export function processPipPlayers(currentPlayerName) {
    const players = fetchPlayers();
    const playerList = document.querySelector("#player-victim-list");
    playerList.innerHTML = "";

    players.forEach((player) => {
        if (player.playerName === currentPlayerName) {
            console.log("Player '" + currentPlayerName + "' removed from pip because he is choosing the victim");
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
}

function processVictimSelection(event) {
    // Єбаний костиль.
    const victim = event.target.parentNode.parentNode;
    console.log("Victim is: " + victim.innerHTML);

    const name = victim.querySelector("p").innerText;
    console.log("Victim is: " + name);
    
    closeModal(document.querySelector("#pig-in-poke-modal"));

    sendPipVictim(name);
}

