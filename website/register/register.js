let first_name = document.getElementById("firstname");
let last_name = document.getElementById("lastname");
let username = document.getElementById("username");
let password = document.getElementById("password");
let password2 = document.getElementById("confirm-password");
let error_msg = document.getElementById("error-message");
let btn_register = document.getElementById("register-button");

document.addEventListener("DOMContentLoaded", function () {
    // <input type="text" id="firstname" placeholder="First Name">
    // <input type="text" id="lastname" placeholder="Last Name">
    // <input type="text" id="username" placeholder="Username">
    // <input type="password" id="password" placeholder="Password">
    // <input type="password" id="confirm-password" placeholder="Confirm Password">
    // <div id="error-message" style="color: red;">Please enter a valid email address.</div>
    // <button id="register-button">Register</button>

    first_name = document.getElementById("firstname");
    last_name = document.getElementById("lastname");
    username = document.getElementById("username");
    password = document.getElementById("password");
    password2 = document.getElementById("confirm-password");
    error_msg = document.getElementById("error-message");
    btn_register = document.getElementById("register-button");


    btn_register.addEventListener('click', () => {
        if(CheckCorrect()){
            // send info to database
        }
    });
});

function CheckCorrect() {
    let f_name = first_name.value;
    let l_name = last_name.value;
    let u_name = username.value;
    let p_name = password.value;
    let p2_name = password2.value;

    let f_err = check_valid_txt(f_name, "The first name field", 1);
    let l_err = check_valid_txt(l_name, "The last name field", 1);
    let u_err = check_valid_txt(u_name, "The username field", 10);
    let p_err = check_valid_txt(p_name, "The password field", 10);

    error_msg.innerHTML = "";
    if (p_name != p2_name) {
        error_msg.innerHTML = "Passwords does not match";
        return;
    }

    if (f_err != " ") {
        error_msg.innerHTML = f_err
    }

    if (l_err != " ") {
        error_msg.innerHTML = l_err
    }

    if (u_err != " ") {
        error_msg.innerHTML = u_err
    }

    if (p_err != " ") {
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