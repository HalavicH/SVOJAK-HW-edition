/////////////    Open      ///////////////
export function openModal(modal) {
    modal.style.display = "block";
    modal.offsetHeight;
    modal.style.opacity = 1;
}

//////////      CLose       /////////////
export function closeModal(modal) {
    modal.style.opacity = 0;

    setTimeout(function () {
        modal.style.display = "none";
    }, 500);
}
