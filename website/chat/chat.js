import { retrive_all_users, Room, Global } from "../shared/database.js"

if (Global.get_uuid() == undefined) {
    window.location.href = "/index.html";
}
if (Global.get_username() == undefined) {
    window.location.href = "/index.html";
}

let current_room_index;
let message_indices = new Map();

let chatBox;
let chatRooms;
let chatContent;
let chatRoomName;
let messageInput;
let sendButton;
let createRoomButton;
let customDialog;
let settingsMenu;
let settingsOptions;
let usernameOptions;
let rooms_parent;
let chatname_input;

document.addEventListener("DOMContentLoaded", async function () {
    // Your JavaScript code here
    // It will be executed when the HTML content is fully loaded.
    chatBox = document.getElementById("chat-box");
    chatRooms = document.querySelectorAll(".chat-room");
    chatContent = document.getElementById("chat-content");
    chatRoomName = document.querySelector(".chat-room-name");
    messageInput = document.getElementById("message-input");
    sendButton = document.getElementById("send-button");
    createRoomButton = document.getElementById("create-room-option");
    customDialog = document.getElementById("custom-dialog");
    settingsMenu = document.getElementById("settings-menu");
    settingsOptions = document.getElementById("settings-options");
    usernameOptions = document.getElementById("username-options")
    rooms_parent = document.getElementById("chat-rooms");
    chatname_input = document.getElementById("chatname-input");
    chatRooms.forEach((room) => {
        room.addEventListener("click", () => {
            on_room_click(room);
        });
    });

    sendButton.addEventListener("click", sendMessage);
    // createRoomButton.addEventListener("click", openCustomDialog);
    createRoomButton.addEventListener("click", openCustomDialog);
    settingsMenu.addEventListener("click", toggleSettingsOptions);

    await sync_rooms();

    setInterval(async function () {
        await sync_rooms();
    }, 2000);
    setInterval(async function () {
        await sync_chat();
    }, 200);
});

async function on_room_click(room) {
    chatRoomName.textContent = room.textContent;
    chatContent.innerHTML = "";

    current_room_index = room[1];
    chatContent.innerHTML = "";


    let messages = await Room.retrieve_messages(current_room_index, 0);
    if (messages.request == "ok") {

        message_indices.set(current_room_index, messages.latest_index);
        messages.messages.forEach((message) => {
            appendMessage(message.username, message.message);
        });
    }
}

async function sync_chat() {

    if (current_room_index == undefined) {
        return;
    }

    let messages = await Room.retrieve_messages(current_room_index, message_indices.get(current_room_index));
    if (messages.request == "ok") {
        message_indices.set(current_room_index, messages.latest_index);
        messages.messages.forEach((message) => {
            appendMessage(message.username, message.message);
        });
    }
    else {
    }
}

async function sync_rooms() {
    let response = await Room.retrieve_all();
    const allKeysArray = Array.from(message_indices.keys());
    if (response.request == "ok") {
        response.chat_rooms.forEach((room) => {

            if (!message_indices.has(room[1])) {

                message_indices.set(room[1], 0);
            }
            else {
                return;
            }

            const newRoom = document.createElement("li");
            newRoom.className = "chat-room";
            newRoom.textContent = room[0];
            newRoom.addEventListener("click", async () => {
                on_room_click(room);
            });
            rooms_parent.appendChild(newRoom);
        })
    }
}

function confirm_pressed() {
    const selectedOptions = usernameOptions.selectedOptions;
    const selectedValues = Array.from(selectedOptions).map(option => option.value);
    let chatname = chatname_input.value;
    if (chatname == "") {
        chatname = "default name"
    }
    Room.create_room(chatname, Global.get_uuid(), selectedValues);

    closeCustomDialog();
}

function sendMessage() {
    const message = messageInput.value;
    if (message.trim() === "") return;
    if (message == undefined) return;

    Room.send_message(current_room_index, message);
    messageInput.value = "";

}

async function openCustomDialog() {
    customDialog.classList.remove("hidden");

    const confirmButton = document.getElementById("confirm-button");
    const cancelButton = document.getElementById("cancel-button");

    // TODO, get all info about users
    let users = await retrive_all_users();
    console.log("first message");
    let username = Global.get_username();
    users.forEach(user => {
        if (user != username) {
            const optionElement = document.createElement("option");
            optionElement.value = user;
            optionElement.textContent = user;
            usernameOptions.appendChild(optionElement);
        }
    });

    confirmButton.addEventListener("click", confirm_pressed);
    cancelButton.addEventListener("click", closeCustomDialog);
}

function closeCustomDialog(confirmFunc) {
    customDialog.classList.add("hidden");

    const confirmButton = document.getElementById("confirm-button");
    const cancelButton = document.getElementById("cancel-button");

    usernameOptions.innerHTML = '';

    confirmButton.removeEventListener("click", confirmFunc);
    cancelButton.removeEventListener("click", closeCustomDialog);
}

function createRoomAndInvite() {
    const roomName = document.getElementById("room-name-input").value;
    const emailAddresses = document.getElementById("email-input").value.split(",").map(email => email.trim());

    if (roomName && emailAddresses.length > 0) {
        // Create the room and invite the users
        const newRoom = document.createElement("li");
        newRoom.className = "chat-room";
        newRoom.textContent = roomName;
        document.getElementById("chat-rooms").appendChild(newRoom);

        emailAddresses.forEach(email => {
            // Send individual invitations to each email address
            const emailSubject = `You've been invited to join ${roomName}`;
            const emailBody = `Click the following link to join the chat room: [insert link here]`;
            const emailLink = `mailto:${email}?subject=${emailSubject}&body=${emailBody}`;
            window.open(emailLink);
        });

        // Close the custom dialog
        closeCustomDialog();
    }
}


function appendMessage(sender, message) {
    // send message to db
    // TODO, send message to db
    if (message == undefined || sender == undefined) {
        return;
    }

    if (sender == Global.get_username()) {
        sender = "You";
    }

    const messageDiv = document.createElement("div");
    messageDiv.innerHTML = `<strong>${sender}:</strong> ${message}`;
    chatContent.appendChild(messageDiv);
    chatContent.scrollTop = chatContent.scrollHeight;
}

function toggleSettingsOptions() {
    if (settingsOptions.style.display === "block") {
        settingsOptions.style.display = "none";
    } else {
        settingsOptions.style.display = "block";
    }
}
