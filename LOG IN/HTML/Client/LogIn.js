root = 'http://127.0.0.1:8090/';

async function LogIn () {
    const RegisterForm = document.getElementById('LogInForm');
    RegisterForm.addEventListener('submit', async function (event) {
        event.preventDefault();
        const data = new FormData(RegisterForm);
        const dataJSON = JSON.stringify(Object.fromEntries(data));
        console.log(dataJSON);
        const response = await fetch(root + 'Log',
        {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
              },
            body: dataJSON
        });
        RegisterForm.reset();
    });
    }

document.addEventListener('DOMContentLoaded', LogIn);