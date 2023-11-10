const chatBox = document.getElementById("chat-box");
const messageInput = document.getElementById("message-input");
const sendButton = document.getElementById("send-button");

sendButton.addEventListener("click", sendMessage);

function sendMessage() {
    const message = messageInput.value;
    if (message.trim() === "") return;

    appendMessage("You", message);
    messageInput.value = "";
}

function appendMessage(sender, message) {
    const messageDiv = document.createElement("div");
    messageDiv.innerHTML = `<strong>${sender}:</strong> ${message}`;
    chatBox.appendChild(messageDiv);

    // Automatically scroll to the bottom to show the latest message
    chatBox.scrollTop = chatBox.scrollHeight;
}