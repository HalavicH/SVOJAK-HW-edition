



export function isImageExisting(imagePath) {
    return false;

    tauri.promisified.fs
        .exists(imagePath)
        .then(exists => {
            if (exists) {
                console.log('Image exists!');
            } else {
                console.log('Image does not exist.');
            }
        })
        .catch(error => {
            console.error('An error occurred while checking image existence:', error);
        });
}