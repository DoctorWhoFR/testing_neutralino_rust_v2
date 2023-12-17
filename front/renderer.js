/**
 * This file is loaded via the <script> tag in the index.html file and will
 * be executed in the renderer process for that window. No Node.js APIs are
 * available in this process because `nodeIntegration` is turned off and
 * `contextIsolation` is turned on. Use the contextBridge API in `preload.js`
 * to expose Node.js functionality from the main process.
 */

document.addEventListener("InitMessage", (e)=>{
    const {id, kind:{InitMessage: {config:{bar_x, bar_y}, state, message}}} = e.detail;

    const bar_x_html = document.getElementById("bar_x");
    bar_x_html.value = bar_x;

    const bar_y_html = document.getElementById("bar_y");
    bar_y_html.value = bar_y;
})

async function bar(pixels){
    const _bar = document.createElement("div");
    _bar.style.display = "flex";
    _bar.style.backgroundColor = "black";
    _bar.style.width = "1000px"
    _bar.style.height = "50px";

    pixels.forEach((pixel)=>{
        const {pala_type} = pixel;
        let _pixel = document.createElement("div")
        _pixel.style.position = "relative";
        _pixel.style.top = "30px";
        _pixel.style.width = "5px";
        _pixel.style.height = "5px";
        _pixel.style.backgroundColor = "white";
        _bar.appendChild(_pixel);
    })

    let cur_bar = document.getElementById("bar");
    cur_bar.innerHTML = "";
    cur_bar.appendChild(_bar)
}

document.addEventListener("PalaBar", (e)=>{
    const {id, kind:{PalaBar: {pixels, cursor, types}}} = e.detail;

    bar(pixels)
})

async function update_config(){
    const bar_x_html = document.getElementById("bar_x");
    const bar_y_html = document.getElementById("bar_y");
    const res = await window.core.handlers.ConfigUpdate({ bar_x:parseInt(bar_x_html.value), bar_y:parseInt(bar_y_html.value) })

    bar_x_html.value = res.bar_x;
    bar_y_html.value = res.bar_y;

    alert("config updated");
}


let config_open = false;

document.addEventListener("mousedown", (e)=>{
    console.log(e)

    if(!config_open) return;

    const bar_x_html = document.getElementById("bar_x");
    const bar_y_html = document.getElementById("bar_y");

    bar_x_html.value = e.screenX;
    bar_y_html.value = e.screenY;

    close_config()
})

document.addEventListener("mousemove", (e)=>{
    if(!config_open) return;

    const box = document.getElementById("box");
    box.style.top = `${e.screenY}px`;
    box.style.left = `${e.screenX}px`;

    document.getElementById("x").innerText = e.screenX;
    document.getElementById("y").innerText = e.screenY;

})

function open_config() {
    const main = document.getElementById("main")
    const config = document.getElementById("config");

    main.style.display = "none";
    config.style.display = "block";
    config_open = true;

    document.addEventListener("keypress", async (e)=>{
        console.log(e)
        if(e.key == "v") {
            await window.core.invokes.Capture(null)

            var timestamp = new Date().getTime();

            var el = document.getElementById("imgd");

            el.src = "test.png?t=" + timestamp;
    }
    })
}

function close_config() {
    const main = document.getElementById("main")
    const config = document.getElementById("config");

    config.style.display = "none";
    main.style.display = "block";
}