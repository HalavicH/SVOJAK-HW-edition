import { fetchPlayers, sendPipVictim } from "../../service/back-end-com.js";
import { closeModal, openModal } from "../../service/modal-common.js";
import { getImagePathOrDefault } from "../../service/utils.js";

const REFS = {
    playerListDiv: document.querySelector("#player-auction-list"),
    auctionModal: document.querySelector("#auction-modal"),
};

export async function processAuctionPlayers(activePlayerId) {
    const players = await fetchPlayers();
    REFS.playerListDiv.innerHTML = "";

    players.forEach((player) => {
        const element = document.createElement("div");
        element.className = "player-auction-badge";

        element.innerHTML = `
            <div class="player-icon">
                <img src=${getImagePathOrDefault(player.playerIconPath)}></>
            </div>
            <div class="player-details">
                <div class="player-details-id" style="display: none;">${player.id}</div>
                <p class="player-details-name">${player.playerName}</p>
                <div class="player-details-bid">Bid: 
                    <p class="player-details-bid-value"> 0 </p>
                </div>
                <div class="player-details-score"> Score: 
                    <p class="player-details-score-value">${player.score}</p>
                <div>
            </div>
        `;

        REFS.playerListDiv.appendChild(element);
    });

    openModal(REFS.auctionModal);

    // TODO: Auction logic
}
