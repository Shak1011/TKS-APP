root = 'http://127.0.0.1:8090/';


async function AddUser () {
    const RegisterForm = document.getElementById('Register-Form');
    RegisterForm.addEventListener('submit', async function (event) {
        event.preventDefault();
        const data = new FormData(RegisterForm);
        const dataJSON = JSON.stringify(Object.fromEntries(data));
        console.log(dataJSON);
        const response = await fetch(root + 'New',
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

document.addEventListener('DOMContentLoaded', AddUser);