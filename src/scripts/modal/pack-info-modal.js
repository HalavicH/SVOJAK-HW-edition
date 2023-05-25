import {openModal, closeModal} from "./modal-common.js"
import {getPackInfo} from "./../back-end-com.js";


export async function openPackInfoModal() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");


    
    openModal(modalPackInfoContainer);
}

export function closePackInfoModal() {
    const modalPackInfoContainer = document.querySelector("#pack-info-modal");

    closeModal(modalPackInfoContainer);
}

