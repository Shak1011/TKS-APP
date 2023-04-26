const ffi = require("ffi-napi")
const lib = ffi.Library("../sqlLog/target/release/hashing",{
    'call_by_node':['str',['str']]
});
let result = lib.CallByNode("yeet");
console.log(result);
