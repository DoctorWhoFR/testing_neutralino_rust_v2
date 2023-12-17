const {execSync} = require("child_process");
const fs = require("fs");

function before_build() {

}

function build() {
    execSync("cargo build")
    console.log("build success")
    fs.renameSync("./target/debug/elec_rust.exe", "../compiled/elec_rust.exe")    


    const t = execSync("neu build", {cwd: "./myapp"})
    console.log("build success", t.toString())
    fs.renameSync("./myapp/dist/myapp/myapp-win_x64.exe", "../compiled/myapp/myapp-win_x64.exe")    
    fs.renameSync("./myapp/dist/myapp/resources.neu", "../compiled/myapp/resources.neu")    
}

function after_build() {

}

build()