export function isImageExisting(imagePath) {
    return false;

    tauri.promisified.fs
        .exists(imagePath)
        .then((exists) => {
            if (exists) {
                console.log("Image exists!");
            } else {
                console.log("Image does not exist.");
            }
        })
        .catch((error) => {
            console.error("An error occurred while checking image existence:", error);
        });
}

export function getImagePathOrDefault(origImagePath) {
    if (origImagePath === undefined 
        || origImagePath === "" 
        || isImageExisting(origImagePath) === false) {
        return "./assets/default-icon.png";
    }
    return origImagePath;
}
