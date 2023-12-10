import { retrive_all_users, Room, Global } from "../shared/database.js"

if (Global.get_uuid() == undefined) {
    window.location.href = "/index.html";
}
if (Global.get_username() == undefined) {
    window.location.href = "/index.html";
}

let current_room_index;
let message_indices = {};

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
    chatRooms.forEach((room) => {
        room.addEventListener("click", () => {
            currentRoom = room.textContent;
            chatRoomName.textContent = currentRoom;
            chatContent.innerHTML = "";
        });
    });

    sendButton.addEventListener("click", sendMessage);
    createRoomButton.addEventListener("click", openCustomDialog("Chatroom", "", confirm_pressed));
    let currentRoom = "Room 1"; // Default room
    settingsMenu.addEventListener("click", toggleSettingsOptions);

    let response = await Room.retrieve_all();
    console.log(response);
    if (response.request == "ok") {
        response.chat_rooms.forEach((room) => {
            const newRoom = document.createElement("li");
            newRoom.className = "chat-room";
            newRoom.textContent = room[0];
            message_indices[room[1]] = 0;
            newRoom.addEventListener("click", async () => {

                chatRoomName.textContent = newRoom.textContent;
                current_room_index = room[1];
                console.log(current_room_index);
                chatContent.innerHTML = "";

                let messages = await Room.retrieve_messages(room[1], message_indices[message_indices[room[1]]])
                if (messages.request == "ok") {
                    message_indices[room[1]] = messages.last_index;
                    messages.messages.forEach((message) => {
                        appendMessage(message.username, message.message);
                    });
                }
                console.log(messages);
            });
            rooms_parent.appendChild(newRoom);
        })
    }
});

function confirm_pressed() {
    const selectedOptions = usernameOptions.selectedOptions;
    const selectedValues = Array.from(selectedOptions).map(option => option.value);
    Room.create_room("testroom", Global.get_uuid(), selectedValues);
}

function sendMessage() {
    const message = messageInput.value;
    if (message.trim() === "") return;
    if (message == undefined) return;

    appendMessage(Global.get_username(), message);
    messageInput.value = "";
}

async function openCustomDialog(title, setup_func, confirmFunc) {
    customDialog.classList.remove("hidden");

    const confirmButton = document.getElementById("confirm-button");
    const cancelButton = document.getElementById("cancel-button");

    // TODO, get all info about users
    let users = await retrive_all_users();
    let username = Global.get_username();
    console.log(username);
    users.forEach(user => {
        if (user != username) {
            const optionElement = document.createElement("option");
            optionElement.value = user;
            optionElement.textContent = user;
            usernameOptions.appendChild(optionElement);
        }
    });

    confirmButton.addEventListener("click", confirmFunc);
    cancelButton.addEventListener("click", closeCustomDialog);
}

function closeCustomDialog(confirmFunc) {
    customDialog.classList.add("hidden");

    const confirmButton = document.getElementById("confirm-button");
    const cancelButton = document.getElementById("cancel-button");

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

    const room_index = current_room_index;

    Room.send_message(room_index, message);

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
