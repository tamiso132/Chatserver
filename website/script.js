const serverIp = '192.168.0.107';
const serverPort = 7878;

const PostEvent = {
    Login: 0,
}

// Function to make an HTTP GET request with custom headers
async function fetchDataWithHeaders(url, customHeaders) {
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

let username = "Tomas";
let password = "Tommers";

let user_send = username.padEnd(50, '\0');
let pass = password.padEnd(50, '\0');

console.log(user_send.length)
console.log(pass.length)
let packet = user_send.concat(pass);

sendDataWithHeaders(PostEvent.Login, packet);
// Function to make an HTTP POST request with custom headers
async function sendDataWithHeaders(url, data, customHeaders) {
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