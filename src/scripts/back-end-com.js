



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
        packName: "Zlyj reper Zenyk",
        packAuthor: "Zlyj reper Zenyk",
        packRounds: 3,
        packTopics: 20,
        packQuestion: 66,
        packTopicList: ["sss", "ooo", "mmm", "bbb", "rrr", "aaa", "Fallout Equestria"]
    }
}
