// ws.js
const ws = new WebSocket("wss://rust-websockets.herokuapp.com:3012");
const contentList = document.getElementsByTagName("ul")[0];
const inpt = document.getElementsByTagName("input")[0];

ws.onopen = attachtEventListener;
ws.onmessage = logAndWrite;

inpt.addEventListener("keydown", evt => evt.keyCode === 13 && send());

function logAndWrite(event) {
  console.log("server:", event.data);
  writeNewMessage(event.data);
}

function writeNewMessage(message) {
  const li = document.createElement("li");
  li.innerHTML = message;
  contentList.appendChild(li);
}

function attachtEventListener() {
  document.getElementsByTagName("a")[0].addEventListener("click", evt => {
    evt.preventDefault();
    send();
  });
  console.log("connection established");
}

function send() {
  ws.send(inpt.value);
  inpt.value = "";
}
