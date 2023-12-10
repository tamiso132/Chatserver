import { login_user, Global } from "../shared/database.js"


let username_div;
let password_div;
document.addEventListener("DOMContentLoaded", function () {

    let button = document.getElementById("register-button");
    let login_btn = document.getElementById("login-button");

    username_div = document.getElementById("username");
    password_div = document.getElementById("password");

    login_btn.addEventListener('click', async () => {
        console.log(username_div);
        let username = username_div.value;
        let password = password_div.value;

        let response = await login_user(username, password);
        console.log(response);
        if (response.request == "ok") {
            let uuid = response.uuid;
            Global.set_uuid(uuid);
            Global.set_username(username);

            console.log("yeppers?");
            window.location.href = "/chat.html";
        }
        else {
            console.log("failed");
            //TODO error message
        }
    })
    button.addEventListener('click', () => {
        window.location.href = "/register/register.html";
    })

})