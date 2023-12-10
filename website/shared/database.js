
export var Global = {
    get_uuid: function () {
        return Number(sessionStorage.getItem("uuid"));
    },
    set_uuid: function (new_val) {

        sessionStorage.setItem("uuid", new_val);
    },

    get_username: function get_username() {
        return sessionStorage.getItem("username");
    },
    set_username: function set_username(new_val) {
        sessionStorage
            .setItem("username", new_val);
    }
}


const PostEvent = {
    Login: 0,
}

export async function login_user(username, password) {

    let user = {
        "request": "login",
        "username": username,
        "password": password,
    };

    let message_back = await send_post("/users.json", JSON.stringify(user));
    return await message_back;
}
export var Room = {
    create_room: async function (chatname, my_uuid, usernames) {
        let data = {
            "request": "create-room",
            "name": chatname,
            "my_uuid": my_uuid,
            "usernames": usernames,
        };
        let response = send_put("/chat_rooms.json", JSON.stringify(data));
    },

    retrieve_all: async function () {
        let data = {
            "request": "get-rooms",
            "uuid": Global.get_uuid(),
        };
        return await send_post("/chat_rooms.json", JSON.stringify(data));
    },

    send_message: async function (room_index, message) {
        if (message == undefined) {
            return;
        }

        let data = {};
        data.request = "add-message";
        data.message = message;
        data.room_index = room_index;
        data.username = Global.get_username();

        send_put("/message.json", data);
    },
    retrieve_messages: async function (room_index, latest_message_index) {

        let data = {
            "request": "get-messages",
            "message_index": latest_message_index,
            "room_index": room_index,
        };
        console.log(room_index, latest_message_index);
        return await send_post("/chat_rooms.json", JSON.stringify(data));
    }

}
export async function retrive_all_users() {
    try {
        const response = await fetch("/get_users.json", {
            method: 'GET',
            headers: {
                // Default headers, if any
                'Content-Type': 'application/json',
                // Custom headers
            },
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }
        let b = await response.json();

        return b.usernames;
    } catch (error) {
        console.error('Error fetching data:', error);
        throw error;
    }
}

// Function to make an HTTP GET request with custom headers
export async function fetchDataWithHeaders(url, customHeaders) {
    try {
        const response = await fetch(url, {
            method: 'GET',
            headers: {
                // Default headers, if any
                'Content-Type': 'application/json',
                // Custom headers
            },
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error('Error fetching data:', error);
        throw error;
    }
}



// Function to make an HTTP POST request with custom headers
export async function send_put(url, data) {
    try {
        console.log(data);
        const response = await fetch(url, {
            method: 'PUT',
            headers: {
                // Default headers, if any
                'Content-Type': 'application/json',
                // Custom headers
            },
            body: data,
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        // Parse the JSON response
        return await response.json();
    } catch (error) {
        console.error('Error sending data:', error);
        throw error;
    }
}
export async function send_post(url, data, customHeaders) {
    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                // Default headers, if any
                'Content-Type': 'application/json',
                // Custom headers
            },
            body: data,
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error('Error sending data:', error);
        throw error;
    }
}

async function postJSON(data) {
    try {
        const response = await fetch("www.example.com", {
            method: "POST", // or 'PUT'
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });

        const result = await response.json();
        console.log("Success:", result);
    } catch (error) {
        console.error("Error:", error);
    }
}

//sendDataWithHeaders("hello", data);