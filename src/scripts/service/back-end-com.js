const {invoke} = window.__TAURI__.tauri;

export async function setHubType(hubType) {
    return await invoke("set_hub_type", {hubType});
}

export async function getSettingsConfig() {
    return await invoke("fetch_configuration");
}

export async function probeHub(selectedOption) {
    return await invoke("discover_hub", {path: selectedOption});
}

export async function setHubRadioChannel(value) {
    return await invoke("set_hub_radio_channel", {
        channelId: parseInt(value),
    });
}

export async function discoverPlayers() {
    return await invoke("discover_players");
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

export async function getPackInfo(path) {
    return invoke("get_pack_info", {path: path});
}

export async function saveRoundDuration(roundDurationMinutes) {
    console.log("Round duration: " + roundDurationMinutes);
    invoke("save_round_duration", {durationMin: roundDurationMinutes});
}

export function startTheGame() {
    return invoke("start_the_game");
}

export async function fetchPlayers() {
    return await invoke("fetch_players");
}

export async function fetchRound() {
    console.log("Getting new round!");
    return await invoke("fetch_round");
}

export async function getQuestionData(topic, price) {
    return await invoke("get_question_data", {topic: topic, price: price});
}

export async function hasNextQuestion() {
    return await invoke("has_next_question");
}

export async function initNextRound() {
    return await invoke("init_next_round");
}

export async function answerQuestion(answeredCorrectly) {
    return await invoke("answer_question", {answeredCorrectly: answeredCorrectly});
}

export async function hasNoPlayerToAnswer() {
    return await invoke("has_no_player_to_answer");
}

export async function sendPipVictim(victimId) {
    console.log(victimId);
    return await invoke("send_pip_victim", {victimId: victimId});
}

export async function getActivePlayerId() {
    return await invoke("get_active_player_id");
}

export async function allowAnswer() {
    return await invoke("allow_answer");
}

export async function finishQuestionPrematurely(topic, price) {
    return await invoke("finish_question_prematurely", {topic: topic, price: price});
}

export async function waitForFirstClick() {
    return await invoke("get_fastest_click");
}

export async function isAllowAnswerRequired() {
    return await invoke("is_allow_answer_required");
}

export async function fetchRoundStats() {
    return await invoke("fetch_round_stats");
    return {
        roundNumber: 2,
        questionNumber: 40,
        normalQuestionNum: 20,
        pigInPokeQuestionNum: 4,
        totalCorrectAnswers: 27,
        totalWrongAnswers: 8,
        roundTime: "13:55",
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
