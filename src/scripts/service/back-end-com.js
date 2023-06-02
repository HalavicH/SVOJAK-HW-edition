const {invoke} = window.__TAURI__.tauri;

export async function getSettingsConfig() {
    return await invoke("fetch_configuration");
}

export async function discoverHub(selectedOption) {
    const result = await invoke("discover_hub", {path: selectedOption});
}

export async function discoverTerminals(channelIdObject) {
    return await invoke("discover_terminals", {
        channelId: parseInt(channelIdObject.value),
    });
}

export async function savePlayers(playersList) {
    playersList.forEach((player) => {
        console.log("Saved player" +
            "id: " + player.terminalId +
            "iconPath: " + player.playerIconPath +
            "name: " + player.playerName +
            "used: " + player.used);
    });

    await invoke("save_players", {players: playersList});
}

export async function saveRoundDuration(roundDurationMinutes) {
    console.log("Round duration: " + roundDurationMinutes);
    invoke("save_round_duration", {durationMin: roundDurationMinutes});
}

export async function getPackInfo(path) {
    return invoke("get_pack_info", {path: path});
}

export async function fetchPlayers() {
    return await invoke("fetch_players");
}

export async function fetchRound() {
    return await invoke("fetch_round");
}

export async function getQuestionData(topic, price) {
    return await invoke("get_question_data", {topic: topic, price: price});
}

export async function hasNextQuestion() {
    return await invoke("has_next_question");
}

export async function getFastestClick() {
    return await invoke("get_fastest_click");

    return {
        newUpdatesPresent: true,
        userWithFastestClick: "Button",
    };
}

export async function answerQuestion(answeredCorrectly) {
    return await invoke("answer_question", {answeredCorrectly: answeredCorrectly});

    return {
        // TODO: Add id and check by id
        id: 1,
        newScore: 666,
    };
}

export async function sendPipVictim(victimId) {
    console.log(victimId);
    return await invoke("send_pip_victim", {victimId: victimId});
}

export async function getActivePlayerId() {
    return await invoke("get_active_player_id");
}

export async function allowAnswer() {

}

export async function waitForFirstClick() {
    return 2;
}

export async function isAllowButtonRequired() {
    return false;
}

export async function getRoundStats() {
    return {
        roundNumber: 1,
        questionNumber: 30,
        normalQuestionNum: 27,
        pigInPokeQuestionNum: 3,
        totalCorrectAnswers: 25,
        totalWrongAnswers: 5,
        roundTime: "13:54",
        players: [
            {
                id: 1,
                name: "HalavicH",
                score: 400,
                playerIconPath: "",
                totalAnswers: 5,
                answeredCorrectly: 3,
                answeredWrong: 2,
            },
            {
                id: 2,
                name: "Button",
                score: 300,
                playerIconPath: "",
                totalAnswers: 5,
                answeredCorrectly: 3,
                answeredWrong: 2,
            },
            {
                id: 3,
                name: "Minty",
                score: 200,
                playerIconPath: "",
                totalAnswers: 5,
                answeredCorrectly: 3,
                answeredWrong: 2,
            }
        ]
    };
}