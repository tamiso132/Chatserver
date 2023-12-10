import {send_put} from "../shared/database.js"

class RegisterUser{
    constructor(firstname, lastname, username, password){
        this.firstname = firstname;
        this.lastname = lastname;
        this.username = username;
        this.password = password;
    }
}

let first_name = document.getElementById("firstname");
let last_name = document.getElementById("lastname");
let username = document.getElementById("username");
let password = document.getElementById("password");
let password2 = document.getElementById("confirm-password");
let error_msg = document.getElementById("error-message");
let btn_register = document.getElementById("register-button");

document.addEventListener("DOMContentLoaded", function () {

    first_name = document.getElementById("firstname");
    last_name = document.getElementById("lastname");
    username = document.getElementById("username");
    password = document.getElementById("password");
    password2 = document.getElementById("confirm-password");
    error_msg = document.getElementById("error-message");
    btn_register = document.getElementById("register-button");


    btn_register.addEventListener('click', async () => {
        if(CheckCorrect()){
            const new_user = new RegisterUser(first_name.value, last_name.value, username.value, password.value);
            new_user.request = "register";
            let message_back = await send_put("/users.json", JSON.stringify(new_user));
            if (message_back.request != "ok"){
                error_msg.textContent = "username already exist";
                return;
            }

            // TODO, go to chat page
        }
    });
});

function CheckCorrect() {
    let f_name = first_name.value;
    let l_name = last_name.value;
    let u_name = username.value;
    let p_name = password.value;
    let p2_name = password2.value;

    let f_err = check_valid_txt(f_name, "The first name field", 5);
    let l_err = check_valid_txt(l_name, "The last name field", 5);
    let u_err = check_valid_txt(u_name, "The username field", 6);
    let p_err = check_valid_txt(p_name, "The password field", 6);

    error_msg.innerHTML = "";
    if (p_name != p2_name) {
        error_msg.innerHTML = "Passwords does not match";
    }

    else if (f_err != " ") {
        error_msg.innerHTML = f_err
    }

    else if (l_err != " ") {
        error_msg.innerHTML = l_err
    }

    else if (u_err != " ") {
        error_msg.innerHTML = u_err
    }

    else if (p_err != " ") {
        error_msg.innerHTML = p_err
    }

    if (error_msg.innerHTML == "") {
        return true;
    }
    return false;
}

function check_valid_txt(content, name, len) {
    if (content == "") {
        return name.concat(" is empty");
    }

    if (content.length < len) {
        return name.concat(" must be at least ", len, " letters");
    }

    return " ";

}