export let uuid = -1;

const PostEvent = {
    Login: 0,
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
export async function send_put(url, data){
    try {
        alert(url);
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