import { fetchPlayers, sendPipVictim } from "../../service/back-end-com.js";
import { closeModal, openModal } from "../../service/modal-common.js";
import { getImagePathOrDefault } from "../../service/utils.js";
import {
    displayQuestionScreen,
    setActivePlayerBadgeState,
    setAllPlayersState,
    updatePlayers,
} from "../gameplay-service.js";

const REFS = {
    playerListDiv: document.querySelector("#player-victim-list"),
    pipModal: document.querySelector("#pig-in-poke-modal"),
};

export async function processPipPlayers(activePlayerId) {
    const players = await fetchPlayers();
    REFS.playerListDiv.innerHTML = "";

    players.forEach((player) => {
        if (player.id === activePlayerId) {
            console.log("Player with id: '" + activePlayerId + "' removed from pip because he is choosing the victim");
            return;
        }

        let playerBadge = document.createElement("div");
        playerBadge.className = "player-victim-badge";
        playerBadge.addEventListener("click", processVictimSelection);
        REFS.playerListDiv.appendChild(playerBadge);

        playerBadge.innerHTML = `
            <div class="player-icon">
                <img src=${getImagePathOrDefault(player.playerIconPath)}/>
            </div>
            <p class="name">${player.playerName}</p>
        `;
        REFS.playerListDiv.appendChild(playerBadge);
    });

    openModal(REFS.pipModal);
}

async function processVictimSelection(event) {
    // Єбаний костиль.
    const victim = event.target.parentNode.parentNode;
    const name = victim.querySelector("p").innerText;
    console.log("Victim is: " + name);

    await sendPipVictim(name);

    closeModal(REFS.pipModal);

    // setPipPlayersSelection();
    updatePlayers();
    displayQuestionScreen();
}

export function setPipPlayersSelection() {
    setAllPlayersState("inactive");
    setActivePlayerBadgeState("target-player");
}
