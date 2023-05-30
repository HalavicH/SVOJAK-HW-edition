const { invoke } = window.__TAURI__.tauri;

export async function getSettingsConfig() {
    return await invoke("fetch_configuration");
}

export async function discoverHub(selectedOption) {
    const result = await invoke("discover_hub", { path: selectedOption });
}

export async function savePlayers(playersList) {
    playersList.forEach((player) => {
        console.log("Saved player" +
        "id: " + player.terminalId +
        "iconPath: " + player.playerIconPath +
        "name: " + player.playerName + 
        "used: " + player.used);
    });

    await invoke("save_players", { players: playersList });
}

export async function saveRoundDuration(roundDurationMinutes) {
    console.log("Round duration: " + roundDurationMinutes);
    const result = await invoke("save_round_duration", { durationMin: roundDurationMinutes });
}

export async function getPackInfo(pathToPack) {
    return await invoke("get_pack_info");
    return {
        packName: "Скрябін",
        packAuthor: "Кузьма",
        packRounds: 1,
        packTopics: 10,
        packQuestion: 16,
        packTopicList: ["фsss", "фooo", "фmmm", "фbbb", "rrr", "aaa", "Fallout Equestria"],
    };
}

export async function fetchPlayers() {
    return [
        {
            id: 1,
            playerIconPath: "./assets/default-icon.png",
            playerName: "Button",
            score: 200,
        },
        {
            id: 2,
            playerIconPath: "",
            playerName: "Button2",
            score: 500,
        },
        {
            id: 3,
            playerIconPath: "",
            playerName: "Button3",
            score: 200,
        },
        {
            id: 4,
            playerIconPath: "",
            playerName: "Button4",
            score: 700,
        },
        {
            id: 5,
            playerIconPath: "",
            playerName: "Button5",
            score: -100,
        },
        {
            id: 6,
            playerIconPath: "",
            playerName: "Button2",
            score: 500,
        },
        {
            id: 7,
            playerIconPath: "",
            playerName: "Button3",
            score: 200,
        },
        {
            id: 8,
            playerIconPath: "",
            playerName: "Button4",
            score: 700,
        },
        {
            id: 9,
            playerIconPath: "",
            playerName: "Жерти",
            score: -100,
        },
    ];
}

export function fetchRound() {
    return {
        roundName: "1",
        roundTopics: [
            {
                topicName: "Festivals",
                questions: {
                    price1: 1000,
                    price2: 2000,
                    price3: 3000,
                    price4: 4000,
                    price5: 5000,
                },
            },
            {
                topicName: "Meme",
                questions: {
                    price1: 1000,
                    price2: 2000,
                    price3: 3000,
                    price4: 4000,
                    price5: 5000,
                },
            },
            {
                topicName: "Amogus",
                questions: {
                    price1: 1000,
                    price2: 2000,
                    price3: 3000,
                    price4: 4000,
                    price5: 5000,
                },
            },
            {
                topicName: "G4",
                questions: {
                    price1: 1000,
                    price2: 2000,
                    price3: 3000,
                    price4: 4000,
                    price5: 5000,
                },
            },
            {
                topicName: "Music",
                questions: {
                    price1: 1000,
                    price2: 2000,
                    price3: 3000,
                    price4: 4000,
                    price5: 5000,
                },
            },
            {
                topicName: "Fanfics",
                questions: {
                    price1: 1000,
                    price2: 2000,
                    price3: 3000,
                    price4: 4000,
                    price5: 5000,
                },
            },
        ],
    };
}

export async function getQuestionData(topic, price) {
    return {
        questionType: "pig-in-poke", // "normal", "pig-in-poke", "auction"
        mediaType: "text", // "text", "video", "music", "picture"
        content: "What is the best pone?",
    };
}

export async function getFastestClick() {
    return {
        newUpdatesPresent: true,
        userWithFastestClick: "Button",
    };
}

export async function answerQuestion(answeredCorrectly) {
    return {
        // TODO: Add id and check by id
        id: 1,
        newScore: 666,
    };
}

export async function sendPipVictim(victimName) {
    console.log(victimName);
}

export async function getActivePlayerId() {
    return 1;
}

export async function allowAnswer() {

}

export async function waitForFirstClick() {
    return 2;
}