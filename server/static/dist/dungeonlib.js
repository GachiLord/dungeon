async function setClipboard(text) {
  const type = "text/plain";
  const blob = new Blob([text], { type });
  const data = [new ClipboardItem({ [type]: blob })];
  await navigator.clipboard.write(data);
}


function showElementById(id) {
  document.getElementById(id).style.display = "block"
}

function showChoice() {
  document.getElementById("choice").style.display = "block";
}

async function inviteUser() {
  const r = await fetch("/api/token", { method: "POST" });
  let b = await r.json();
  setClipboard(b.token);
}

function nextLine(callback) {
  const lines = document.getElementById("dialog-text").children;
  let found = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (line.style.display !== "none" && i != lines.length - 1) {
      found = true;
      line.style.display = "none";
    } else if (found) {
      line.style.display = "block";
      if (callback && i == lines.length - 1) callback()
      break;
    }
  }

}
