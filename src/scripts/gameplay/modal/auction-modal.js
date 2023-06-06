import { fetchPlayers, sendPipVictim } from "../../service/back-end-com.js";
import { closeModal, openModal } from "../../service/modal-common.js";
import { getImagePathOrDefault } from "../../service/utils.js";


export async function processAuctionPlayers(activePlayerId) {
    const players = await fetchPlayers();
    const playerList = document.querySelector("#player-auction-list");
    playerList.innerHTML = "";

    players.forEach((player) => {
        let playerBadge = document.createElement("div");
        playerBadge.className = "player-auction-badge";
        // playerBadge.addEventListener("click", processVictimSelection);
        playerBadge.style.cursor = "pointer";
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
    
        let playerDetailsId = document.createElement("div");
        playerDetailsId.className = "player-details-id";
        playerDetailsId.style.display = "none";
        playerDetailsId.innerText = player.id;
        playersDetails.appendChild(playerDetailsId);
    
        let playerDetailsName = document.createElement("p");
        playerDetailsName.className = "player-details-name";
        playerDetailsName.innerText = player.playerName;
        playersDetails.appendChild(playerDetailsName);

        let playerBid = document.createElement("div");
        playerBid.className = "player-details-bid";
        playerBid.innerText = "Bid: ";
        playersDetails.appendChild(playerBid);

        let bid = document.createElement("p");
        bid.className = "player-details-bid-value";
        bid.innerText = 0;
        playerBid.appendChild(bid);
    
        let playerDetailsScore = document.createElement("div");
        playerDetailsScore.className = "player-details-score";
        playerDetailsScore.innerText = "Score: ";
        playersDetails.appendChild(playerDetailsScore);
    
        let score = document.createElement("p");
        score.className = "player-details-score-value";
        score.innerText = player.score;
        playerDetailsScore.appendChild(score);    
    });

    const modal = document.querySelector("#auction-modal");

    openModal(modal);

    // TODO: Auction logic
}

