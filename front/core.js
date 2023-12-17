const core = {
    handlers: {},
    events: {},
    invokes: {},
    receiver_connected: false,
}

const receiver = new WebSocket("ws://localhost:8080");

const messages = [];

core.receiver = receiver;
core.messages = messages;

function invoke(l_event){
    receiver.send(JSON.stringify(l_event))
}
core.invoke = invoke;

async function Handle(l_event) {
    receiver.send(JSON.stringify(l_event))

    return new Promise((resolve, reject) => {
        const responseInterval = setInterval(() => {
            const res = messages.find(msg => msg.key === l_event.key);

            if(res){
                clearInterval(responseInterval);
                resolve(res);
            } else {
                console.log("No response", messages);
            }
        }, 1000);

    });
}

core.Handle = Handle;

receiver.addEventListener("open", async function (event) {
    core.receiver_connected = true;
});

receiver.addEventListener("message", function (event) {
    let msg = JSON.parse(event.data);
    console.log("[CORE]", msg)

    let optional_event = window.core.events[msg.id];

    if(optional_event){
        const event_re = new CustomEvent(optional_event, {
            detail: msg
        });
        document.dispatchEvent(event_re);
    } else {
        messages.push(msg);
    }
});

window.core = core;


window.core.events.InitMessage = "InitMessage";

window.core.events.PalaBar = "PalaBar";



window.core.handlers.ConfigUpdate = async (kind) => new Promise(async (resolve) => {
     const res = await window.core.Handle({  id: "ConfigUpdate", kind: {  "type": "ConfigUpdate" } })

     resolve(res)
 })



window.core.invokes.StartBot = (kind) => window.core.invoke({  id: "StartBot", kind: {  "type": "StartBot" } })

window.core.invokes.StopBot = (kind) => window.core.invoke({  id: "StopBot", kind: {  "type": "StopBot" } })

window.core.invokes.Capture = (kind) => window.core.invoke({  id: "Capture", kind: {  "type": "Capture" } })


console.log("Core system V0.1")
