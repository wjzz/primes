const root = document.getElementById("root");
const payload = document.getElementById("payload");

root.innerText = "Hello from JS!";

const loadData = async () => {
    const response = await fetch("json");
    const data = await response.json();

    console.log("Got data", data);
    payload.innerHTML =`<pre>${JSON.stringify(data, "", 2)}</pre>`;
}

const loadData2 = async () => {
    const response = await fetch("count");
    const data = await response.text();

    root.innerHTML = "";

    const divs = data.split("\n\n");
    divs.forEach(div => {
        const el = document.createElement("div");
        el.innerHTML = div.split("\n").join("<br />");
        root.append(el);
    });

    console.log("Got data", data);
    // root.innerHTML = txt;
}

loadData();
loadData2();

setInterval(loadData2, 5000);