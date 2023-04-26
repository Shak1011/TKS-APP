const express = require('express');
const app = express();
const fs = require('fs');

app.use(express.json());
const path = require('path');
app.use(express.static(path.join(__dirname, 'client')));

const ffi = require("ffi-napi");
const { info } = require('console');
const lib = ffi.Library("../sqlLog/target/release/hashing",{
    'add_user':['[u3]',['str']],
    'comfirm_user':['[u2]',['str']],
});


//sends user info to json file
app.post('/New', function (req, resp) {
    lib.add_user(req.body)
    resp.send("User Created");
});

app.post('/Log', function (req, resp) {
    let INFO = lib.comfirm_user(req.body)
    resp.send(INFO);
});

module.exports = app;
