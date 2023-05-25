



export function getSettingsConfig() {
    return {
        hubStatus: "Detected",
        hubPort: "/dev/tty3",
        availablePorts: ["/dev/tty1", "/dev/tty2", "/dev/tty3"],
        // radioChannel: "",
        players: [
            {
                terminalId: 35,
                playerIconPath: "./assets/default-icon.png",
                playerName: "Button",
                used: true
            },
            {
                terminalId: 34,
                playerIconPath: "",
                playerName: "Button2",
                used: true
            },
            {
                terminalId: 33,
                playerIconPath: "",
                playerName: "Button3",
                used: true
            },
            {
                terminalId: 32,
                playerIconPath: "",
                playerName: "Button4",
                used: true
            },
            {
                terminalId: 31,
                playerIconPath: "",
                playerName: "Button5",
                used: true
            }
        ]
    }
}

export function getPackInfo(pathToPack) {
    return {
        packName: "Скрябін",
        packAuthor: "Кузьма",
        packRounds: 1,
        packTopics: 10,
        packQuestion: 16,
        packTopicList: ["фsss", "фooo", "фmmm", "фbbb", "rrr", "aaa", "Fallout Equestria"]
    }
}
