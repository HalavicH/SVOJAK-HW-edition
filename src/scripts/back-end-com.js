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

export function fetchPlayers() {
    return [
        {
            playerIconPath: "./assets/default-icon.png",
            playerName: "Button",
            score: 200,
        },
        {
            playerIconPath: "",
            playerName: "Button2",
            score: 500,
        },
        {
            playerIconPath: "",
            playerName: "Button3",
            score: 200,
        },
        {
            playerIconPath: "",
            playerName: "Button4",
            score: 700,
        },
        {
            playerIconPath: "",
            playerName: "Button5",
            score: -100,
        },
        {
            playerIconPath: "",
            playerName: "Button2",
            score: 500,
        },
        {
            playerIconPath: "",
            playerName: "Button3",
            score: 200,
        },
        {
            playerIconPath: "",
            playerName: "Button4",
            score: 700,
        },
        {
            playerIconPath: "",
            playerName: "Button5",
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
        questionType: "normal", // "normal", "pig-in-poke", "auction"
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

export async function getUpdatedScores(answeredCorrectly) {
    return {
        userName: "HalavicH",
        newScore: 666,
    };
}

export async function sendPipVictim(victimName) {
    console.log(victimName);
}
