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

document.addEventListener("DOMContentLoaded", function () {
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

    chatRooms.forEach((room) => {
        room.addEventListener("click", () => {
            currentRoom = room.textContent;
            chatRoomName.textContent = currentRoom;
            chatContent.innerHTML = "";
        });
    });

    sendButton.addEventListener("click", sendMessage);
    createRoomButton.addEventListener("click", openCustomDialog);
    let currentRoom = "Room 1"; // Default room
    settingsMenu.addEventListener("click", toggleSettingsOptions);
});

function sendMessage() {
    const message = messageInput.value;
    if (message.trim() === "") return;

    appendMessage("You", message);
    messageInput.value = "";
}

function openCustomDialog(title, setup_func, confirmFunc) {
    customDialog.classList.remove("hidden");

    const confirmButton = document.getElementById("confirm-button");
    const cancelButton = document.getElementById("cancel-button");

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


class Friend {
    constructor(username, index) {
        this.username = username;
        this.index = index;
    }
}
function createFriendList() {

    let friends = [new Friend("Tom", 0), new Friend("Beluga", 1)]
    createFriendInDialog(friends);

}



function createFriendList(friend_list) {
    const friendList = document.createElement("ul");
    friendList.className = "friend-list";

    // Define the list of friends
    friend_list.forEach((friend) => {
        const listItem = document.createElement("li");

        const checkbox = document.createElement("input");
        checkbox.type = "checkbox";
        checkbox.className = "friend-checkbox";
        checkbox.id = friend.id;
        checkbox.value = friend.name;

        const label = document.createElement("label");
        label.htmlFor = friend.id;
        label.textContent = friend.name;

        listItem.appendChild(checkbox);
        listItem.appendChild(label);

        friendList.appendChild(listItem);
    });

    document.getElementsByClassName("modal-content")[0].appendChild(friendList); // Append the friend list to the document body or a container of your choice
}

